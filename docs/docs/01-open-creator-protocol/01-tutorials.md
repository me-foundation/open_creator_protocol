---
title: Tutorials
description: "Describes how developer can get started with the Open Creator Protocol."
---

## How to mint OCP NFT
TODO (add rust and ts examples)

## How to lock/unlock OCP NFT for trading/staking/loan
TODO (add rust and ts examples)

## How to create a policy
TODO (add rust and ts examples)

## How to update a policy
TODO (add rust and ts examples)

## How to run OCP locally (for development)

Clone the repository to your local machine:

```shell
git clone git@github.com:magiceden-oss/open_creator_protocol.git
```

Setup, build, and test the codebase:

```shell
# Install deps
npm i

# To build and generate the solitarc
./build.sh

# To test
anchor test
```

### Using the CLI

The CLI can be executed by running the following from the project directory:

```shell
ts-node sdk/src/cli.ts
```

```bash
CLI_COMMAND=create_policy \
CLI_AUTHORITY=./keypair.json \
CLI_RPC=https://api.devnet.solana.com \
CLI_JSON_RULE='{"conditions":{"field":"action","operator":"string_not_equals","value":""},"events":[]}' \
  ts-node sdk/src/cli.ts
```

:::tip Concepts
View the [Concepts](02-concepts.md) guide for better understanding of each term.
:::
