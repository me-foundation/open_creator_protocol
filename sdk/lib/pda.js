"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.findFreezeAuthorityPk = exports.findMintStatePk = exports.findPolicyPk = exports.CMT_PROGRAM = void 0;
const anchor_1 = require("@project-serum/anchor");
const web3_js_1 = require("@solana/web3.js");
const pubkey_1 = require("@project-serum/anchor/dist/cjs/utils/pubkey");
const generated_1 = require("./generated");
exports.CMT_PROGRAM = new web3_js_1.PublicKey("CMTQqjzH6Anr9XcPVt73EFDTjWkJWPzH7H6DtvhHcyzV");
const findPolicyPk = (uuid) => {
    return (0, pubkey_1.findProgramAddressSync)([
        anchor_1.utils.bytes.utf8.encode("policy"),
        uuid.toBuffer(),
    ], generated_1.PROGRAM_ID)[0];
};
exports.findPolicyPk = findPolicyPk;
const findMintStatePk = (mint) => {
    return (0, pubkey_1.findProgramAddressSync)([anchor_1.utils.bytes.utf8.encode("mint_state"), mint.toBuffer()], generated_1.PROGRAM_ID)[0];
};
exports.findMintStatePk = findMintStatePk;
const findFreezeAuthorityPk = (policy) => {
    return (0, pubkey_1.findProgramAddressSync)([policy.toBuffer()], exports.CMT_PROGRAM)[0];
};
exports.findFreezeAuthorityPk = findFreezeAuthorityPk;
