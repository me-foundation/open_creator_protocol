import { utils } from "@project-serum/anchor";
import { Connection, Keypair, PublicKey, Transaction, TransactionInstruction } from "@solana/web3.js";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { PROGRAM_ID } from "./generated";

export const CMT_PROGRAM = new PublicKey("CMTQqjzH6Anr9XcPVt73EFDTjWkJWPzH7H6DtvhHcyzV");

export const findPolicyPk = (
  uuid: PublicKey,
) => {
  return findProgramAddressSync(
    [
      utils.bytes.utf8.encode("policy"),
      uuid.toBuffer(),
    ],
    PROGRAM_ID
  )[0];
};

export const findMintStatePk = (mint: PublicKey) => {
  return findProgramAddressSync(
    [utils.bytes.utf8.encode("mint_state"), mint.toBuffer()],
    PROGRAM_ID
  )[0];
};

export const findFreezeAuthorityPk = (policy: PublicKey) => {
  return findProgramAddressSync(
    [policy.toBuffer()],
    CMT_PROGRAM,
  )[0];
};

export const process_tx = async (
  conn: Connection,
  ixs: TransactionInstruction[],
  signers: Keypair[],
): Promise<string> => {
  const tx = new Transaction();
  tx.feePayer = signers[0].publicKey;
  tx.recentBlockhash = (await conn.getLatestBlockhash()).blockhash;
  tx.add(...ixs);
  tx.partialSign(...signers);
  try {
    const sig = await conn.sendRawTransaction(tx.serialize());
    console.log({ sig });
    await new Promise((r) => setTimeout(r, 1000));
    return sig;
  } catch (e) {
    console.error(e);
    throw e;
  }
};