---
title: Getting Started
description: "Describes how developer can get started with the Open Creator Protocol."
---

# Getting Started

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

## Using the CLI

The CLI can be executed by running the following from the project directory:

```shell
ts-node sdk/src/cli.ts
```

### Configuration

Configure your deployment using the following environment variables as an example:

```bash
CLI_COMMAND=create_policy \
CLI_AUTHORITY=./keypair.json \
CLI_RPC=https://api.devnet.solana.com \
CLI_JSON_RULE='{"conditions":{"field":"action","operator":"string_not_equals","value":""},"events":[]}' \
```

Then execute the configuration with:

```shell
ts-node sdk/src/cli.ts
```

:::tip Configuration
View the [Configuration](02-configuration.md) guide for more deployment options.
:::

## Dynamic royalties

Creators can specify a relationship between an NFT’s sale price and royalty amount via a linear price curve. And more
curve types to be supported in the future. The first dynamic royalty curve OCP supports is `DynamicRoyaltyPriceLinear`.
Both `start_multiplier_bp` and `end_multiplier_bp` are relative multipliers based on the
Metaplex's `metadata.seller_fee_basis_points`.

```rust
pub struct DynamicRoyaltyPriceLinear {
    pub price_mint: Option<Pubkey>,
    pub start_price: u64,
    pub end_price: u64,
    pub start_multiplier_bp: u16,
    pub end_multiplier_bp: u16,
}
```
Specifically, if we note that r is the final multiplier_bp, then:

$$
r = y1 + (y2 - y1) * (price - x1) / (x2 - x1)
$$

For example, given the following dynamic royalty setting:

```bash
DynamicRoyaltyPriceLinear:
{
  start_price: 0 SOL
  end_price: 5 SOL
  start_multiplier_bp: 100% (10000 bp)
  end_multiplier_bp: 50% (5000 bp)
}

Metadata:
seller_fee_basis_points: 5% (500 bp)

price: 0 SOL   ===> royalty_bp: 500 (5%)
price: 2.5 SOL ===> royalty_bp: 375 (3.75%)
price: 5 SOL   ===> royalty_bp: 250 (2.5%)
```
