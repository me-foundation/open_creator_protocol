import { utils } from "@project-serum/anchor";
import {
  ComputeBudgetProgram,
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { DynamicRoyalty, PROGRAM_ID } from "./generated";
import * as anchor from "@project-serum/anchor";

export const LARGER_COMPUTE_UNIT = 1_400_000;
export const computeBudgetIx = ComputeBudgetProgram.setComputeUnitLimit({ units: LARGER_COMPUTE_UNIT });

export const CMT_PROGRAM = new PublicKey(
  "CMTQqjzH6Anr9XcPVt73EFDTjWkJWPzH7H6DtvhHcyzV"
);

export const findPolicyPk = (uuid: PublicKey) => {
  return PublicKey.findProgramAddressSync(
    [utils.bytes.utf8.encode("policy"), uuid.toBuffer()],
    PROGRAM_ID
  )[0];
};

export const findMintStatePk = (mint: PublicKey) => {
  return PublicKey.findProgramAddressSync(
    [utils.bytes.utf8.encode("mint_state"), mint.toBuffer()],
    PROGRAM_ID
  )[0];
};

export const findFreezeAuthorityPk = (policy: PublicKey) => {
  return PublicKey.findProgramAddressSync([policy.toBuffer()], CMT_PROGRAM)[0];
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
      startPrice,
      endPrice,
      startMultiplierBp,
      endMultiplierBp,
    },
    reserved0: new Array(32).fill(0),
    reserved1: new Array(32).fill(0),
    reserved2: new Array(32).fill(0),
    reserved3: new Array(32).fill(0),
  };
  return dynamicRoyalty;
};

export const parsePriceLinearDynamicRoyaltyStruct = (jsonStr: string) => {
  if (jsonStr === "" || jsonStr === "null") {
    return null;
  }

  const {startPrice, endPrice, startMultiplierBp, endMultiplierBp} = JSON.parse(jsonStr);
  return createDynamicRoyaltyStruct({
    startPrice: new anchor.BN(startPrice),
    endPrice: new anchor.BN(endPrice),
    startMultiplierBp: Number(startMultiplierBp),
    endMultiplierBp: Number(endMultiplierBp),
  });
}

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
    await conn.confirmTransaction(sig);
    return sig;
  } catch (e) {
    console.error(e);
    throw e;
  }
};
