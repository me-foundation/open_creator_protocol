import * as anchor from "@project-serum/anchor";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

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
