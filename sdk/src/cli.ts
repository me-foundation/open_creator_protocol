import {
  Connection,
  Keypair,
  PublicKey,
  SYSVAR_INSTRUCTIONS_PUBKEY,
} from "@solana/web3.js";
import {
  createInitPolicyInstruction,
  createMigrateToMplInstruction,
  createUpdatePolicyInstruction,
} from "./generated";
import {
  CMT_PROGRAM,
  findFreezeAuthorityPk,
  findMintStatePk,
  findPolicyPk,
  parsePriceLinearDynamicRoyaltyStruct,
  process_tx,
} from "./pda";
import fs from "fs";
import {
  findMasterEditionV2Pda,
  findMetadataPda,
  TokenMetadataProgram,
} from "@metaplex-foundation/js";

const CLI_COMMAND: "create_policy" | "update_policy" | "migrate_to_mpl" =
  (process.env.CLI_COMMAND ?? "create_policy") as any;
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
const CLI_DYNAMIC_ROYALTY_PRICE_LINEAR = process.env
  .CLI_DYNAMIC_ROYALTY_PRICE_LINEAR
  ? parsePriceLinearDynamicRoyaltyStruct(
      process.env.CLI_DYNAMIC_ROYALTY_PRICE_LINEAR
    )
  : null;
const CLI_POLICY_PUBKEY = new PublicKey(
  process.env.CLI_POLICY_PUBKEY ?? Keypair.generate().publicKey
);
const CLI_MINT = new PublicKey(
  process.env.CLI_MINT ?? Keypair.generate().publicKey
);
const CLI_UPDATE_AUTHORITY = Keypair.fromSecretKey(
  Buffer.from(
    JSON.parse(
      fs.readFileSync(process.env.CLI_UPDATE_AUTHORITY ?? "./keypair.json", {
        encoding: "utf-8",
      })
    )
  )
);

const conn = new Connection(CLI_RPC, "confirmed");

async function create_policy() {
  const uuid = Keypair.generate().publicKey;
  const ix = createInitPolicyInstruction(
    {
      policy: findPolicyPk(uuid),
      authority: CLI_AUTHORITY.publicKey,
      uuid,
    },
    {
      arg: {
        jsonRule: CLI_JSON_RULE,
        dynamicRoyalty: CLI_DYNAMIC_ROYALTY_PRICE_LINEAR,
      },
    }
  );
  await process_tx(conn, [ix], [CLI_AUTHORITY]);
  console.log("policy uuid: ", uuid.toBase58());
  console.log("policy created: ", findPolicyPk(uuid).toBase58());
}

async function update_policy() {
  const ix = createUpdatePolicyInstruction(
    { policy: CLI_POLICY_PUBKEY, authority: CLI_AUTHORITY.publicKey },
    {
      arg: {
        authority: CLI_AUTHORITY.publicKey,
        jsonRule: CLI_JSON_RULE,
        dynamicRoyalty: CLI_DYNAMIC_ROYALTY_PRICE_LINEAR,
      },
    }
  );
  await process_tx(conn, [ix], [CLI_AUTHORITY]);
  console.log("policy updated: ", CLI_POLICY_PUBKEY.toBase58());
}

async function migrate_to_mpl() {
  const tokenAccount = (await conn.getTokenLargestAccounts(CLI_MINT)).value[0]
    .address;
  const ix = createMigrateToMplInstruction({
    policy: CLI_POLICY_PUBKEY,
    freezeAuthority: findFreezeAuthorityPk(CLI_POLICY_PUBKEY),
    mint: CLI_MINT,
    metadata: findMetadataPda(CLI_MINT),
    mintState: findMintStatePk(CLI_MINT),
    from: CLI_UPDATE_AUTHORITY.publicKey,
    fromAccount: tokenAccount,
    cmtProgram: CMT_PROGRAM,
    instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
    edition: findMasterEditionV2Pda(CLI_MINT),
    metadataProgram: TokenMetadataProgram.publicKey,
    payer: CLI_UPDATE_AUTHORITY.publicKey,
  });
  await process_tx(conn, [ix], [CLI_UPDATE_AUTHORITY]);
  console.log("migrated to mpl, mint: ", CLI_MINT.toBase58());
}

async function run() {
  switch (CLI_COMMAND) {
    case "create_policy":
      await create_policy();
      break;
    case "update_policy":
      await update_policy();
      break;
    case "migrate_to_mpl":
      await migrate_to_mpl();
      break;
  }
}

// main entrypoint
if (typeof require !== "undefined" && require.main === module) {
  run();
}
