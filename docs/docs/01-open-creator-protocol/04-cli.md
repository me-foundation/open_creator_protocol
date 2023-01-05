---
title: CLI
description: "Describes the cli usage of the Open Creator Protocol."
---

# CLI Usage

The CLI can be executed by running the following from the project directory:

```shell
git clone https://github.com/magiceden-oss/open_creator_protocol.git
cd open_creator_protocol
ts-node sdk/src/cli.ts
```

Create a policy without a dynamic royalty setting:

```bash
CLI_COMMAND=create_policy \
CLI_AUTHORITY=./keypair.json \
CLI_RPC=https://api.devnet.solana.com \
CLI_JSON_RULE='{"conditions":{"field":"action","operator":"string_not_equals","value":""},"events":[]}' \
  ts-node sdk/src/cli.ts
```

Create a policy with a dynamic royalty setting:

```bash
CLI_COMMAND=create_policy \
CLI_AUTHORITY=./keypair.json \
CLI_RPC=https://api.devnet.solana.com \
CLI_JSON_RULE='{"conditions":{"field":"action","operator":"string_not_equals","value":""},"events":[]}' \
CLI_DYNAMIC_ROYALTY_PRICE_LINEAR='{"startPrice":0,"endPrice":5000000000,"startMultiplierBp":10000,"endMultiplierBp":0}' \
  ts-node sdk/src/cli.ts
```

Update a policy

```bash
CLI_POLICY_PUBKEY=TODO \
CLI_COMMAND=update_policy \
CLI_AUTHORITY=./keypair.json \
CLI_RPC=https://api.devnet.solana.com \
CLI_JSON_RULE='{"conditions":{"field":"action","operator":"string_not_equals","value":""},"events":[]}' \
CLI_DYNAMIC_ROYALTY_PRICE_LINEAR='{"startPrice":0,"endPrice":5000000000,"startMultiplierBp":10000,"endMultiplierBp":0}' \
  ts-node sdk/src/cli.ts
```

Migrate OCP to MPL

```bash
CLI_COMMAND=migrate_to_mpl \
CLI_POLICY_PUBKEY=TODO \
CLI_MINT=TODO \
CLI_UPDATE_AUTHORITY=./metadata_update_authority_keypair.json \
CLI_RPC=https://api.devnet.solana.com \
  ts-node sdk/src/cli.ts
```
