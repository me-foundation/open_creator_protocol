import { Connection, PublicKey, SystemProgram } from "@solana/web3.js";
import { Transaction } from "@solana/web3.js";
import {
  createAssociatedTokenAccountInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  getMinimumBalanceForRentExemptMint,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

export const createMintTx = async (
  connection: Connection,
  mint: PublicKey,
  authority: PublicKey
) => {
  const ata = getAssociatedTokenAddressSync(mint, authority);
  return new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: authority,
      newAccountPubkey: mint,
      space: MINT_SIZE,
      lamports: await getMinimumBalanceForRentExemptMint(connection),
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeMint2Instruction(mint, 0, authority, authority),
    createAssociatedTokenAccountInstruction(authority, ata, authority, mint),
    createMintToInstruction(mint, ata, authority, 1)
  );
};
