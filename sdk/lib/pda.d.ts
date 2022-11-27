import { PublicKey } from "@solana/web3.js";
export declare const CMT_PROGRAM: PublicKey;
export declare const findPolicyPk: (uuid: PublicKey) => PublicKey;
export declare const findMintStatePk: (mint: PublicKey) => PublicKey;
export declare const findFreezeAuthorityPk: (policy: PublicKey) => PublicKey;
