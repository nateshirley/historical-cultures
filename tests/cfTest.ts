import * as anchor from "@project-serum/anchor";
import * as web3 from "@solana/web3.js";
import {
  Token,
  TOKEN_PROGRAM_ID,
  MintLayout,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { BN, Program } from "@project-serum/anchor";
import { Cultures } from "../target/types/cultures";
import { CollectionFactory } from "../target/types/collection_factory";

import { PublicKey, SystemProgram } from "@solana/web3.js";
import {
  createAssociatedTokenAccountInstruction,
  findAssociatedTokenAccount,
} from "./helpers/tokenHelpers";

declare var TextEncoder: any;

describe("cultures", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const anyAnchor: any = anchor;
  const Cultures = anyAnchor.workspace.Cultures as Program<Cultures>;
  const CollectionFactory = anyAnchor.workspace
    .CollectionFactory as Program<CollectionFactory>;

  interface Pda {
    address: web3.PublicKey;
    bump: number;
  }
  let collection: Pda;
  let factoryAuthority: Pda;

  it("initialize", async () => {
    collection = await findCollection("test");
    factoryAuthority = await findCollectionFactoryAuthority();

    const tx = await CollectionFactory.rpc.initialize(factoryAuthority.bump, {
      accounts: {
        initializer: provider.wallet.publicKey,
        factoryAuthority: factoryAuthority.address,
        systemProgram: SystemProgram.programId,
      },
    });
  });

  it("create collection", async () => {
    const tx = await CollectionFactory.rpc.createCollection(
      "test",
      collection.bump,
      300,
      "T",
      provider.wallet.publicKey,
      10,
      [{ address: provider.wallet.publicKey, share: 100 }],
      100,
      {
        accounts: {
          creator: provider.wallet.publicKey,
          collection: collection.address,
          systemProgram: SystemProgram.programId,
        },
      }
    );
  });

  it("mint into the collection", async () => {
    let itemMint = web3.Keypair.generate();
    let itemTokenAccount = await findAssociatedTokenAccount(
      provider.wallet.publicKey,
      itemMint.publicKey
    );
    const tx = await CollectionFactory.rpc.mintIntoCollection({
      accounts: {
        collection: collection.address,
        itemMint: itemMint.publicKey,
        payer: provider.wallet.publicKey,
        receiver: provider.wallet.publicKey,
        receiverTokenAccount: itemTokenAccount.address,
        collectionMintAuthority: provider.wallet.publicKey,
        factoryAuthority: factoryAuthority.address,
        rent: web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      },
      signers: [itemMint],
    });

    let newCollection = await CollectionFactory.account.collection.fetch(
      collection.address
    );
    console.log(newCollection);
    let tA = await provider.connection.getTokenAccountBalance(
      itemTokenAccount.address
    );
    console.log(tA);
  });

  const findCollectionFactoryAuthority = async () => {
    return PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("f_auth")],
      CollectionFactory.programId
    ).then(([address, bump]) => {
      return {
        address: address,
        bump: bump,
      };
    });
  };
  const findCollection = async (name: string) => {
    return PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("collection"),
        anchor.utils.bytes.utf8.encode(name),
      ],
      CollectionFactory.programId
    ).then(([address, bump]) => {
      return {
        address: address,
        bump: bump,
      };
    });
  };
});
