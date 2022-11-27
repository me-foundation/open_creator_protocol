import { utils } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
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
