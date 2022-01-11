import {
  PublicKey,
  TransactionInstruction,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Keypair,
} from "@solana/web3.js";
import * as SPLToken from "@solana/spl-token";
const { TOKEN_PROGRAM_ID } = SPLToken;

const ASSOCIATED_PROGRAM_ID = new PublicKey(
  "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
);

interface Pda {
  address: PublicKey;
  bump: number;
}

export const createAssociatedTokenAccountInstruction = (
  mint: PublicKey,
  associatedAccount: PublicKey,
  owner: PublicKey,
  payer: PublicKey
) => {
  const data = Buffer.alloc(0);
  let keys = [
    { pubkey: payer, isSigner: true, isWritable: true },
    { pubkey: associatedAccount, isSigner: false, isWritable: true },
    { pubkey: owner, isSigner: false, isWritable: false },
    { pubkey: mint, isSigner: false, isWritable: false },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
  ];
  return new TransactionInstruction({
    keys,
    programId: ASSOCIATED_PROGRAM_ID,
    data,
  });
};

export const findAssociatedTokenAccount = async (
  owner: PublicKey,
  mint: PublicKey
) => {
  let associatedProgramId = new PublicKey(
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
  );
  return PublicKey.findProgramAddress(
    [owner.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    associatedProgramId
  ).then(([address, bump]) => {
    return {
      address: address,
      bump: bump,
    };
  });
};
