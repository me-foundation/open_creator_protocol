---
title: Concepts
description: "Describes the concepts of Open Creator Protocol."
---

# Concepts

OCP uses [SPL managed-token](https://github.com/solana-labs/solana-program-library/tree/master/managed-token) as the
base for holding the freeze authority of the spl-token. And managed-token has 1:1
feature parity of the spl-token in terms of the token interfaces like `transfer`, `approve`, `revoke`, `burn`, `close`,
`init_account` and `mint_to`.

<img src={'../img/high_level.png'} alt={'high level design'} style={{borderRadius: '0px'}}/>

## Action Context

Action context is the context used to build the json rules engine. OCP's solution is a JSON rules engine DSL to define
what can be applied to the context.

```rust
pub struct ActionCtx {
    pub action: String,
    pub program_ids: Vec<String>,
    pub mint: String,
    pub mint_state: MintStateCtx,
    pub mint_account: Option<MintAccountCtx>,
    pub metadata: Option<MetadataCtx>,
    pub payer: Option<String>,
    pub from: Option<String>, // owner of the from_account, and many action's initiator
    pub to: Option<String>,   // owner of the to_account
    pub last_memo_signer: Option<String>,
    pub last_memo_data: Option<String>,
}
```

<img src={'../img/policy_account.png'} alt={'policy engine'} style={{borderRadius: '0px'}}/>

## Onchain Policy Engine

OCP utilises the JSON Rules Engine package, for more information [please see the repository](https://github.com/GopherJ/json-rules-engine-rs).

| Sample Use Cases | Policy (json_rule)  |
| ----------- | ----------- |
| Allow For All | null |
| Program IDs Allowlist | `{ "field": "program_ids", "operator": "string_is_subset", "value": ["1111111111111111111111111111111"]}` |
| Program IDs Denylist | `{ "field": "program_ids", "operator": "string_does_not_contain_any", "value": ["1111111111111111111111111111111"]}` |
| Soulbound Token | `{ "field": "mint_state/transferred_count", "operator": "int_less_than", "value": 1 }` |
| Semi Soulbound Token | `{ "field": "mint_state/transferred_count", "operator": "int_less_than", "value": n }` |
| Transfer Timestamp Constraint | `{ "field": "mint_state/derived_datetime/utc_timestamp", "operator": "int_greater_than", "value": 1669881409}` |
| Transfer Cooldown Token | `{ "field": "mint_state/derived_cooldown", "operator": "int_greater_than", "value": 3600 }` |
| Metadata Name Filter | `{ "field": "metadata/name", "operator": "string_has_substring", "value": "FROZEN"}` |
| Metadata URI Filter | `{ "field": "metadata/uri", "operator": "string_has_substring", "value": "IPFS"}` |
| Single Transfer Destination | `{ "field": "to", "operator": "string_equals", "value": ["1111111111111111111111111111111"]}` |

:::note Transfer Logic

Here's a full example of how a creator can leverage OCP to personalize the transferability. The logic works like this:

- When the `action` is not `transfer`, pass
- When the `action` is `transfer`, then one cannot transfer if the `metadata/name` contains a keyword `FROZEN`
- When the `action` is `transfer`, then one cannot transfer to a specific address if the `metadata/name` doesn't contain `WINNER`.

```json
{
  "events": [],
  "conditions": {
    "or": [
      {
        "field": "action",
        "operator": "string_not_equals",
        "value": "transfer"
      },
      {
        "and": [
          {
            "not": {
              "field": "metadata/name",
              "operator": "string_has_substring",
              "value": "FROZEN"
            }
          },
          {
            "or": [
              {
                "field": "to",
                "operator": "string_not_equals",
                "value": "DWuopEsTrg5qWMSMVT1hoiVTRQG9PkGJZSbXiKAxHYbn"
              },
              {
                "field": "metadata/name",
                "operator": "string_has_substring",
                "value": "WINNER"
              }
            ]
          }
        ]
      }
    ]
  }
}
```

:::

## Mint State

`MintState` determines if a mint (token) is with OCP or not. Mint state is a key PDA that OCP uses to associate a mint account with a policy and some state information related to the mint account.

```rust
pub struct MintState {
    pub version: u8,
    pub bump: [u8; 1],
    pub mint: Pubkey,
    pub policy: Pubkey,
    pub locked_by: Option<Pubkey>,
    pub last_approved_at: i64,
    pub last_transferred_at: i64,
    pub transferred_count: u32,
}
```

One can use the `findMintStatePk` to test if a mint account has a `MintState` account, and then leverage what OCP program provides.

```js
export const findMintStatePk = (mint: PublicKey) => {
  return PublicKey.findProgramAddressSync(
    [utils.bytes.utf8.encode("mint_state"), mint.toBuffer()],
    PROGRAM_ID
  )[0];
};
```

## Dynamic Royalties

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
