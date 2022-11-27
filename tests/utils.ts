import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, Transaction, TransactionInstruction } from "@solana/web3.js";

export const airdrop = async (
  connection: Connection,
  to: PublicKey,
  amount: number
) => {
  await connection.confirmTransaction({
    ...(await connection.getLatestBlockhash()),
    signature: await connection.requestAirdrop(to, amount * LAMPORTS_PER_SOL),
  });
};

export const process_tx = async (
  connection: Connection,
  ixs: TransactionInstruction[],
  signers: Keypair[]
) => {
  const tx = new Transaction();
  tx.feePayer = signers[0].publicKey;
  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
  tx.add(...ixs);
  tx.partialSign(...signers);
  try {
    const sig = await connection.sendRawTransaction(tx.serialize());
    console.log({ sig });
    await new Promise((r) => setTimeout(r, 500));
  } catch (e) {
    console.error(e);
    throw e;
  }
};
