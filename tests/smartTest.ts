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
import { SmartCollections } from "../target/types/smart_collections";

import { PublicKey, SystemProgram } from "@solana/web3.js";
import {
  findTokenMetadata,
  findAssociatedTokenAccount,
  findMasterEdition,
  TOKEN_METADATA_PROGRAM_ID,
} from "./helpers/tokenHelpers";

declare var TextEncoder: any;

describe("cultures", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const anyAnchor: any = anchor;
  const Cultures = anyAnchor.workspace.Cultures as Program<Cultures>;
  const SmartCollections = anyAnchor.workspace
    .SmartCollections as Program<SmartCollections>;

  interface Pda {
    address: web3.PublicKey;
    bump: number;
  }
  let collection: Pda;
  let smartAuthority: Pda;
  let collectionMetadata: Pda;
  let collectionMasterEdition: Pda;
  let collectionMint = web3.Keypair.generate();

  it("initialize", async () => {
    collection = await findSmartCollection("test");
    smartAuthority = await findSmartCollectionsAuthority();
    collectionMetadata = await findTokenMetadata(collectionMint.publicKey);
    collectionMasterEdition = await findMasterEdition(collectionMint.publicKey);
    const tx = await SmartCollections.rpc.initialize(smartAuthority.bump, {
      accounts: {
        initializer: provider.wallet.publicKey,
        smartAuthority: smartAuthority.address,
        systemProgram: SystemProgram.programId,
      },
    });
  });

  it("create collection", async () => {
    let collectionTokenAccount = await findAssociatedTokenAccount(
      smartAuthority.address,
      collectionMint.publicKey
    );
    const tx = await SmartCollections.rpc.createCollection(
      "test",
      collection.bump,
      300,
      "T",
      "https://nateshirley.github.io/data/default.json",
      provider.wallet.publicKey,
      10,
      [{ address: provider.wallet.publicKey, share: 100 }],
      100,
      {
        accounts: {
          payer: provider.wallet.publicKey,
          smartCollection: collection.address,
          collectionMint: collectionMint.publicKey,
          collectionMetadata: collectionMetadata.address,
          collectionMasterEdition: collectionMasterEdition.address,
          collectionTokenAccount: collectionTokenAccount.address,
          smartAuthority: smartAuthority.address,
          rent: web3.SYSVAR_RENT_PUBKEY,
          tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        },
        signers: [collectionMint],
      }
    );
  });

  it("mint into the collection", async () => {
    let itemMint = web3.Keypair.generate();
    let itemTokenAccount = await findAssociatedTokenAccount(
      provider.wallet.publicKey,
      itemMint.publicKey
    );
    let itemMetadata = await findTokenMetadata(itemMint.publicKey);
    let itemMasterEdition = await findMasterEdition(itemMint.publicKey);
    const tx = await SmartCollections.rpc.mintItemIntoCollection(
      null,
      null,
      "gooogle.com",
      {
        accounts: {
          smartCollection: collection.address,
          itemMint: itemMint.publicKey,
          itemMetadata: itemMetadata.address,
          itemMasterEdition: itemMasterEdition.address,
          payer: provider.wallet.publicKey,
          receiver: provider.wallet.publicKey,
          receiverTokenAccount: itemTokenAccount.address,
          collectionMintAuthority: provider.wallet.publicKey,
          smartAuthority: smartAuthority.address,
          rent: web3.SYSVAR_RENT_PUBKEY,
          tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        },
        signers: [itemMint],
      }
    );

    let newCollection = await SmartCollections.account.smartCollection.fetch(
      collection.address
    );
    console.log(newCollection);
    let tA = await provider.connection.getTokenAccountBalance(
      itemTokenAccount.address
    );
    console.log(tA);
  });

  const findSmartCollectionsAuthority = async () => {
    return PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("s_auth")],
      SmartCollections.programId
    ).then(([address, bump]) => {
      return {
        address: address,
        bump: bump,
      };
    });
  };
  const findSmartCollection = async (name: string) => {
    return PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("collection"),
        anchor.utils.bytes.utf8.encode(name),
      ],
      SmartCollections.programId
    ).then(([address, bump]) => {
      return {
        address: address,
        bump: bump,
      };
    });
  };
});
