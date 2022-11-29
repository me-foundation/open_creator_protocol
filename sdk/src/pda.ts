import { utils } from "@project-serum/anchor";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { DynamicRoyalty, PROGRAM_ID } from "./generated";
import * as anchor from "@project-serum/anchor";

export const CMT_PROGRAM = new PublicKey(
  "CMTQqjzH6Anr9XcPVt73EFDTjWkJWPzH7H6DtvhHcyzV"
);

export const findPolicyPk = (uuid: PublicKey) => {
  return findProgramAddressSync(
    [utils.bytes.utf8.encode("policy"), uuid.toBuffer()],
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
  return findProgramAddressSync([policy.toBuffer()], CMT_PROGRAM)[0];
};

export const createDynamicRoyaltyStruct = ({
  startPrice,
  endPrice,
  startMultiplierBp,
  endMultiplierBp,
}: {
  startPrice: anchor.BN;
  endPrice: anchor.BN;
  startMultiplierBp: number;
  endMultiplierBp: number;
}): DynamicRoyalty => {
  const dynamicRoyalty = {
    version: 0,
    kind: 0,
    overrideRoyaltyBp: null,
    kindPriceLinear: {
      priceMint: null,
      startPrice: new anchor.BN(LAMPORTS_PER_SOL),
      endPrice: new anchor.BN(LAMPORTS_PER_SOL * 2),
      startMultiplierBp: 10000,
      endMultiplierBp: 5000,
    },
    reserved0: new Array(32).fill(0),
    reserved1: new Array(32).fill(0),
    reserved2: new Array(32).fill(0),
    reserved3: new Array(32).fill(0),
  };
  return dynamicRoyalty;
};

export const process_tx = async (
  conn: Connection,
  ixs: TransactionInstruction[],
  signers: Keypair[]
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
