import * as anchor from "@project-serum/anchor";
import * as web3 from "@solana/web3.js";
import { Program } from "@project-serum/anchor";
import { Cultures } from "../target/types/cultures";
import { PermissionlessAuction } from "../target/types/permissionless_auction";

describe("cultures", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const anyAnchor: any = anchor;
  const cultures = anyAnchor.workspace.Cultures as Program<Cultures>;
  const auction = anyAnchor.workspace
    .PermissionlessAuction as Program<PermissionlessAuction>;

  interface Pda {
    address: web3.PublicKey;
    bump: number;
  }
  let culture: Pda;

  it("culture init!", async () => {
    // Add your test here.

    let [address, bump] = await web3.PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("culture")],
      cultures.programId
    );
    culture = {
      address: address,
      bump: bump,
    };

    const tx = await cultures.rpc.initialize(bump, {
      accounts: {
        payer: provider.wallet.publicKey,
        culture: address,
        systemProgram: web3.SystemProgram.programId,
      },
    });
    console.log("Your transaction signature", tx);
  });

  it("auction init", async () => {
    const tx2 = await auction.rpc.initialize(culture.bump, {
      accounts: {
        culture: culture.address,
      },
    });
  });
});
