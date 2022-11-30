import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import {
  createInitPolicyInstruction,
  createUpdatePolicyInstruction,
} from "./generated";
import { findPolicyPk, process_tx } from "./pda";
import fs from "fs";

const CLI_COMMAND: "create_policy" | "update_policy"  = (process.env
  .CLI_COMMAND ?? "create_policy") as any;
const CLI_AUTHORITY = Keypair.fromSecretKey(
  Buffer.from(
    JSON.parse(
      fs.readFileSync(process.env.CLI_AUTHORITY ?? "./keypair.json", {
        encoding: "utf-8",
      })
    )
  )
);
const CLI_RPC = process.env.CLI_RPC ?? "https://api.devnet.solana.com";
const CLI_JSON_RULE =
  process.env.CLI_JSON_RULE ??
  JSON.stringify({
    events: [],
    conditions: { field: "action", operator: "string_not_equals", value: "" },
  });
const CLI_DYNAMIC_ROYALTY =
  process.env.CLI_DYNAMIC_ROYALTY ?
  JSON.parse(process.env.CLI_DYNAMIC_ROYALTY) : null;
const CLI_POLICY_PUBKEY = new PublicKey(process.env.CLI_POLICY_PUBKEY ?? Keypair.generate().publicKey);

const conn = new Connection(CLI_RPC, "confirmed");

async function create_policy() {
  const uuid = Keypair.generate().publicKey;
  const ix = createInitPolicyInstruction(
    {
      policy: findPolicyPk(uuid), authority: CLI_AUTHORITY.publicKey,
      uuid,
    },
    { arg: { jsonRule: CLI_JSON_RULE, dynamicRoyalty: CLI_DYNAMIC_ROYALTY} }
  );
  await process_tx(conn, [ix], [CLI_AUTHORITY]);
  console.log("policy uuid: ", uuid.toBase58());
  console.log("policy created: ", findPolicyPk(uuid).toBase58());
}

async function update_policy() {
  const ix = createUpdatePolicyInstruction(
    { policy: CLI_POLICY_PUBKEY, authority: CLI_AUTHORITY.publicKey },
    { arg: { authority: CLI_AUTHORITY.publicKey, jsonRule: CLI_JSON_RULE, dynamicRoyalty: CLI_DYNAMIC_ROYALTY } }
  );
  await process_tx(conn, [ix], [CLI_AUTHORITY]);
  console.log("policy updated: ", CLI_POLICY_PUBKEY.toBase58());
}

async function run() {
  switch (CLI_COMMAND) {
    case "create_policy":
      await create_policy();
      break;
    case "update_policy":
      await update_policy();
      break;
  }
}

// main entrypoint
if (typeof require !== "undefined" && require.main === module) {
  run();
}
