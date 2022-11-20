import { utils } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { PROGRAM_ID } from "./generated";

export const findPolicyPk = (
  update_authority: PublicKey,
  update_authority_nonce = 0
) => {
  return findProgramAddressSync(
    [
      utils.bytes.utf8.encode("policy"),
      update_authority.toBuffer(),
      Uint8Array.from([update_authority_nonce]),
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

export const findFreezeAuthorityPk = (mint: PublicKey) => {
  return findProgramAddressSync(
    [findMintStatePk(mint).toBuffer()],
    new PublicKey("CMTQqjzH6Anr9XcPVt73EFDTjWkJWPzH7H6DtvhHcyzV")
  )[0];
};
