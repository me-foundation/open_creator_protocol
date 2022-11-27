import * as anchor from "@project-serum/anchor";
import { Keypair, TransactionInstruction } from "@solana/web3.js";
import { assert } from "chai";
import {
  createInitPolicyInstruction,
  createUpdatePolicyInstruction,
  Policy,
} from "../sdk/src/generated";
import { findPolicyPk } from "../sdk/src/pda";
import { airdrop, process_tx } from "./utils";

describe("policy", () => {
  const { connection } = anchor.AnchorProvider.env();
  const uuid = Keypair.generate().publicKey;
  const provider = new anchor.AnchorProvider(
    connection,
    new anchor.Wallet(Keypair.generate()),
    {
      commitment: "confirmed",
    }
  );
  const alice = Keypair.generate();
  const bob = Keypair.generate();
  const eve = Keypair.generate();

  beforeEach(async () => {
    await airdrop(provider.connection, alice.publicKey, 50);
    await airdrop(provider.connection, bob.publicKey, 50);
    await airdrop(provider.connection, eve.publicKey, 50);
  });

  describe("Can create policy", () => {
    it("happy path", async () => {
      const jsonRule = JSON.stringify({
        events: [],
        conditions: {
          and: [{ field: "action", operator: "string_not_equals", value: "" }],
        },
      });
      const ix = createInitPolicyInstruction(
        { policy: findPolicyPk(uuid), authority: alice.publicKey },
        { arg: { uuid, jsonRule } }
      );
      await process_tx(provider.connection, [ix], [alice]);
      const policy = await Policy.fromAccountAddress(
        provider.connection,
        findPolicyPk(uuid)
      );
      assert.isTrue(policy.authority.equals(alice.publicKey));
    });

    it("big payload ok", async () => {
      // about 12 "AND" rules before hitting memory limit
      const jsonRule = JSON.stringify({
        events: [],
        conditions: {
          and: Array(12).fill({ field: "action", operator: "string_not_equals", value: "" }),
        },
      });
      const ix = createInitPolicyInstruction(
        { policy: findPolicyPk(uuid), authority: alice.publicKey },
        { arg: { uuid, jsonRule } }
      );
      await process_tx(provider.connection, [ix], [alice]);
      const policy = await Policy.fromAccountAddress(
        provider.connection,
        findPolicyPk(uuid)
      );
      assert.isTrue(policy.authority.equals(alice.publicKey));
    });

    it("big payload with many pubkeys", async () => {
      // about 18 pubkeys before hitting payload limit
      const jsonRule = JSON.stringify({
        events: [],
        conditions: {
          and: [{ field: "program_ids", operator: "string_is_subset", value: Array(18).fill(Keypair.generate().publicKey) }],
        },
      });
      const ix = createInitPolicyInstruction(
        { policy: findPolicyPk(uuid), authority: alice.publicKey },
        { arg: { uuid, jsonRule } }
      );
      await process_tx(provider.connection, [ix], [alice]);
      const policy = await Policy.fromAccountAddress(
        provider.connection,
        findPolicyPk(uuid)
      );
      assert.isTrue(policy.authority.equals(alice.publicKey));
    });
  });

  describe("Can update policy", () => {
    it("alice set bob as the authority", async () => {
      let ix: TransactionInstruction;
      const jsonRule = JSON.stringify({
        events: [],
        conditions: {
          and: [{ field: "action", operator: "string_not_equals", value: "" }],
        },
      });
      ix = createUpdatePolicyInstruction(
        { policy: findPolicyPk(uuid), authority: alice.publicKey },
        { arg: { authority: bob.publicKey, jsonRule } }
      );
      await process_tx(provider.connection, [ix], [alice]);
      {
        const policy = await Policy.fromAccountAddress(
          provider.connection,
          findPolicyPk(uuid)
        );
        assert.isTrue(policy.authority.equals(bob.publicKey));
      }

      ix = createUpdatePolicyInstruction(
        { policy: findPolicyPk(uuid), authority: bob.publicKey },
        { arg: { authority: alice.publicKey, jsonRule } }
      );
      await process_tx(provider.connection, [ix], [bob]);
      {
        const policy = await Policy.fromAccountAddress(
          provider.connection,
          findPolicyPk(uuid)
        );
        assert.isTrue(policy.authority.equals(alice.publicKey));
      }

      // same ix should fail because bob is not the authority
      try {
        await process_tx(provider.connection, [ix], [bob]);
        assert.fail("should have failed");
      } catch (e: any) {
        assert.include(e.message, "Transaction simulation failed");
      }
    });
  });
});
