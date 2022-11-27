import * as anchor from "@project-serum/anchor";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, Transaction, TransactionInstruction } from "@solana/web3.js";

export const conn = new anchor.AnchorProvider(
  anchor.AnchorProvider.env().connection,
  new anchor.Wallet(Keypair.generate()),
  { commitment: "confirmed" }
).connection;

export const airdrop = async (
  to: PublicKey,
  amount: number
) => {
  await conn.confirmTransaction({
    ...(await conn.getLatestBlockhash()),
    signature: await conn.requestAirdrop(to, amount * LAMPORTS_PER_SOL),
  });
};

export const process_tx = async (
  ixs: TransactionInstruction[],
  signers: Keypair[],
) => {
  const tx = new Transaction();
  tx.feePayer = signers[0].publicKey;
  tx.recentBlockhash = (await conn.getLatestBlockhash()).blockhash;
  tx.add(...ixs);
  tx.partialSign(...signers);
  try {
    const sig = await conn.sendRawTransaction(tx.serialize());
    console.log({ sig });
    await new Promise((r) => setTimeout(r, 1000));
  } catch (e) {
    console.error(e);
    throw e;
  }
};
