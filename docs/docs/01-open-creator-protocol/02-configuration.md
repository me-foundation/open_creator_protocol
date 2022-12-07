---
title: Configuration
description: "Describes the configuration options of Open Creator Protocol."
---

# Configuration

Create a policy without a dynamic royalty setting:

```bash
CLI_COMMAND=create_policy \
CLI_AUTHORITY=./keypair.json \
CLI_RPC=https://api.devnet.solana.com \
CLI_JSON_RULE='{"conditions":{"field":"action","operator":"string_not_equals","value":""},"events":[]}' \
```

Create a policy with a dynamic royalty setting:

```bash
CLI_COMMAND=create_policy \
CLI_AUTHORITY=./keypair.json \
CLI_RPC=https://api.devnet.solana.com \
CLI_JSON_RULE='{"conditions":{"field":"action","operator":"string_not_equals","value":""},"events":[]}' \
CLI_DYNAMIC_ROYALTY_PRICE_LINEAR='{"startPrice":0,"endPrice":5000000000,"startMultiplierBp":10000,"endMultiplierBp":0}' \
```

Update a policy

```bash
CLI_POLICY_PUBKEY=TODO \
CLI_COMMAND=update_policy \
CLI_AUTHORITY=./keypair.json \
CLI_RPC=https://api.devnet.solana.com \
CLI_JSON_RULE='{"conditions":{"field":"action","operator":"string_not_equals","value":""},"events":[]}' \
CLI_DYNAMIC_ROYALTY_PRICE_LINEAR='{"startPrice":0,"endPrice":5000000000,"startMultiplierBp":10000,"endMultiplierBp":0}' \
```
### JSON Rules

OCP utilises the JSON Rules Engine package, for more information [please see the repository](https://github.com/GopherJ/json-rules-engine-rs).

### Example Rules

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

:::

### Example JSON

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
