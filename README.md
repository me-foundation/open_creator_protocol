# Open Creator Protocol

Open Creator Protocol is an open protocol for creators to build utilities and the policy engine for their tokens.

Onchain Accounts

| Network | Type | Address  |
| ----------- | ----------- | ----- |
| Devnet  | Program   | ocp4vWUzA2z2XMYJ3QhM9vWdyoyoQwAFJhRdVTbvo9E |
| Devnet  | Policy (allow all) | BSpZkG5dJ5EBhTQmkjiyELAQuKyMVEqrW6DVu53zi9kU |

| Network | Type | Address  |
| ----------- | ----------- | ----- |
| Mainnet | Program   | ocp4vWUzA2z2XMYJ3QhM9vWdyoyoQwAFJhRdVTbvo9E |

## Architecture

Overview

<img src="./docs/arch.excalidraw.png" width="400">

Policy

<img src="./docs/policy.excalidraw.png" width="400">

Dynamic Royalty

<img src="./docs/dynamic_royalty.excalidraw.png" width="400">

## Development

```bash
# Install deps
npm i

# To build and generate the solitarc
./build.sh

# To test
anchor test

# To create a policy
CLI_COMMAND=create_policy \
CLI_AUTHORITY=./keypair.json \
CLI_RPC=https://api.devnet.solana.com \
CLI_JSON_RULE='{"conditions":{"field":"action","operator":"string_not_equals","value":""},"events":[]}' \
  ts-node sdk/src/cli.ts

# To update a policy
CLI_COMMAND=update_policy \
CLI_AUTHORITY=./keypair.json \
CLI_RPC=https://api.devnet.solana.com \
CLI_JSON_RULE='{"conditions":{"field":"action","operator":"string_not_equals","value":""},"events":[]}' \
CLI_POLICY_PUBKEY=... \
  ts-node sdk/src/cli.ts
```

## Licenses
- Apache 2.0
