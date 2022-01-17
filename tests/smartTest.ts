import * as anchor from "@project-serum/anchor";
import * as web3 from "@solana/web3.js";
import {
  Token,
  TOKEN_PROGRAM_ID,
  MintLayout,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { BN, Program, Provider } from "@project-serum/anchor";
//import { Cultures } from "../target/types/cultures";
import { SmartCollections } from "../target/types/smart_collections";

import {
  PublicKey,
  SystemProgram,
  Connection,
  Commitment,
  Keypair,
} from "@solana/web3.js";
import {
  findTokenMetadata,
  findAssociatedTokenAccount,
  findMasterEdition,
  TOKEN_METADATA_PROGRAM_ID,
} from "./helpers/tokenHelpers";
import { getSmartCollectionsProgram } from "./helpers/config";
import { decodeMetadataV2 } from "./helpers/metadata/decodeMetadata";

declare var TextEncoder: any;

describe("cultures", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const anyAnchor: any = anchor;
  //const Cultures = anyAnchor.workspace.Cultures as Program<Cultures>;
  const SmartCollections = getSmartCollectionsProgram(provider.wallet);

  interface Pda {
    address: web3.PublicKey;
    bump: number;
  }
  let collection: Pda;
  let smartAuthority: Pda;
  let collectionMetadata: Pda;
  let collectionMasterEdition: Pda;
  let newCollectionMintPair = web3.Keypair.generate();
  let collectionMint: PublicKey;
  let collectionName = "test13";

  let initialize = false;
  let createCollection = false;
  let mintItem = true;

  it("config", async () => {
    collection = await findSmartCollection(collectionName);
    smartAuthority = await findSmartCollectionsAuthority();
    if (createCollection) {
      collectionMetadata = await findTokenMetadata(
        newCollectionMintPair.publicKey
      );
      collectionMasterEdition = await findMasterEdition(
        newCollectionMintPair.publicKey
      );
    } else {
      let collectionInfo = await SmartCollections.account.smartCollection.fetch(
        collection.address
      );
      collectionMint = collectionInfo.mint;
      collectionMetadata = await findTokenMetadata(collectionMint);
      collectionMasterEdition = await findMasterEdition(collectionMint);
    }
  });

  if (initialize) {
    it("initialize", async () => {
      const tx = await SmartCollections.rpc.initialize(smartAuthority.bump, {
        accounts: {
          initializer: provider.wallet.publicKey,
          smartAuthority: smartAuthority.address,
          systemProgram: SystemProgram.programId,
        },
      });
    });
  }

  //so if i were to cpi into this, i would set the authority for the culture to this
  if (createCollection) {
    it("create collection", async () => {
      let collectionTokenAccount = await findAssociatedTokenAccount(
        smartAuthority.address,
        newCollectionMintPair.publicKey
      );
      const tx = await SmartCollections.rpc.createCollection(
        collectionName,
        collection.bump,
        300,
        "T",
        "https://nateshirley.github.io/data/default.json",
        provider.wallet.publicKey,
        10,
        [
          { address: smartAuthority.address, share: 0 },
          { address: provider.wallet.publicKey, share: 100 },
        ], //still need to decide what i'm going to do for this but this is how u need to do it -- update auth needs to be a creator as well
        100,
        {
          accounts: {
            payer: provider.wallet.publicKey,
            smartCollection: collection.address,
            collectionMint: newCollectionMintPair.publicKey,
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
          signers: [newCollectionMintPair],
        }
      );
    });
  }

  const printKey = (key: PublicKey) => {
    console.log(key.toBase58());
  };

  if (mintItem) {
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
            collectionMint: collectionMint,
            collectionMetadata: collectionMetadata.address,
            collectionMasterEdition: collectionMasterEdition.address,
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

      let metadataInfo = await provider.connection.getAccountInfo(
        itemMetadata.address
      );
      console.log(metadataInfo);
      let decoded = decodeMetadataV2(metadataInfo.data);
      console.log(decoded);
    });
  }

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
