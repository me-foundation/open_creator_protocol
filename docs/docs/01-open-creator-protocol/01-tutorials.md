---
title: Tutorials
description: "Describes how developer can get started with the Open Creator Protocol."
---

## How to interact with OCP NFT

For javascript sdk examples, please take a look at the
[tests/token.spec.ts](https://github.com/magiceden-oss/open_creator_protocol/blob/main/tests/token.spec.ts).

```js
import { createTransferInstruction, process_tx } from "@magiceden-oss/open_creator_protocol";

const transferIx = createTransferInstruction({
  policy: DEVNET_POLICY_ALL,
  freezeAuthority: findFreezeAuthorityPk(DEVNET_POLICY_ALL),
  mint: tokenMint,
  metadata: findMetadataPda(tokenMint),
  mintState: findMintStatePk(tokenMint),
  from: alice.publicKey,
  fromAccount: aliceAta,
  cmtProgram: CMT_PROGRAM,
  instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
  to: bob.publicKey,
  toAccount: bobAta,
});

await process_tx(conn, [computeBudgetIx, initAccIx, transferIx], [alice]);
```

For solana programs using cpi, here's an example of doing the transfer.

```toml
[dependencies]
open_creator_protocol = { version = "0.2.9", features = ["cpi"] }
```

```rust
open_creator_protocol::cpi::transfer(CpiContext::new(
    ctx.accounts.ocp_program.to_account_info(),
    open_creator_protocol::cpi::accounts::TransferCtx {
        policy: ctx.accounts.ocp_policy.to_account_info(),
        mint: ctx.accounts.token_mint.to_account_info(),
        metadata: ctx.accounts.metadata.to_account_info(),
        mint_state: ctx.accounts.ocp_mint_state.to_account_info(),
        from: ctx.accounts.program_as_signer.to_account_info(),
        from_account: ctx.accounts.seller_token_ata.to_account_info(),
        cmt_program: ctx.accounts.cmt_program.to_account_info(),
        instructions: ctx.accounts.instructions.to_account_info(),
        freeze_authority: ctx.accounts.ocp_freeze_authority.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        to: ctx.accounts.buyer.to_account_info(),
        to_account: ctx.accounts.buyer_token_ata.to_account_info(),
    },
))?;
```

## How to mint OCP NFT

To mint an OCP NFT please follow these instructions.

1. Create or use an existing policy account
1. Create mint account
2. Create metadata account
3. Call `wrap` in OCP to wrap the mint account with the policy
4. Call `init_account` in OCP to create the token account ata
5. Call `mint_to` in OCP to actually mint into the token account using the mint account.

For more details, please take a look at the [util.ts example](https://github.com/magiceden-oss/open_creator_protocol/blob/8064939f234c5453b3a6bed108aec729803232ad/tests/utils.ts#L62).

## How to lock/unlock OCP NFT for trading/staking/loan

OCP NFT provides a convinent way of "lock/unlock" as a higher order of freezing.
For example, if we want to implement escrowless listing in a marketplace. This is also true for any staking or loan platform wants
to have escrowless lock-in of user tokens.

```
// List
1. Call `approve` in OCP to setup the delegate just like normal tokens
2. Call `lock` in OCP to lock the token. It prevents the OCP token from accepting any other actions

// Delist
1. Call `unlock` in OCP to unlock the token
2. Call `revoke` in OCP to revoke the previously approved delegate
```

## How to create a policy

```js
const jsonRule = JSON.stringify({
  events: [],
  conditions: {
    and: [{ field: "action", operator: "string_not_equals", value: "" }],
  },
});
const ix = createInitPolicyInstruction(
  {
    policy: findPolicyPk(uuid),
    authority: alice.publicKey,
    uuid,
  },
  { arg: { jsonRule, dynamicRoyalty: null } }
);
await process_tx(conn, [ix], [alice]);
```

## How to update a policy
```js
const jsonRule = JSON.stringify({
  events: [],
  conditions: {
    and: [{ field: "action", operator: "string_not_equals", value: "" }],
  },
});
ix = createUpdatePolicyInstruction(
  { policy: findPolicyPk(uuid), authority: alice.publicKey },
  { arg: { authority: bob.publicKey, jsonRule, dynamicRoyalty: null } }
);
await process_tx(conn, [ix], [alice]);
```

## How to migrate OCP token back to other standards (e.g. Metaplex Master Edition)

Notice that OCP NFTs are based on `spl-managed-token`, `spl-token`, and `token metadata` programs.

- Token Mint (supply = 1, decimals = 0)
- Token Account
- Token Metadata

By definition, it's the same implementation of all the NFTs on solana. Everything is the same including
interacting with wallets (except "transfer", that users can use ME profile page to send tokens including OCP NFTs),
run token gated content, and prove token ownerships exactly like the normal Normal NFTs.

OCP provides an upstream authority to interact with `spl-managed-token` that wraps the token interfaces.

And if creators want to migrate from OCP to other standards, the seamless way of doing that is to call
one of the migration entrypoints in OCP. Example of how to run the migration with `update_authority` of
the metadata can be found in this [PR](https://github.com/magiceden-oss/open_creator_protocol/pull/49).

```js
TODO
```

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
