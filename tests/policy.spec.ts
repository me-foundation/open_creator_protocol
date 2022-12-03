import * as anchor from "@project-serum/anchor";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  TransactionInstruction
} from "@solana/web3.js";
import { assert } from "chai";
import {
  createInitPolicyInstruction,
  createUpdatePolicyInstruction,
  Policy
} from "../sdk/src/generated";
import {
  createDynamicRoyaltyStruct,
  findPolicyPk,
  process_tx
} from "../sdk/src/pda";
import { airdrop, conn } from "./utils";

describe("policy", () => {
  const uuid = Keypair.generate().publicKey;
  const alice = Keypair.generate();
  const bob = Keypair.generate();
  const eve = Keypair.generate();

  beforeEach(async () => {
    await airdrop(alice.publicKey, 50);
    await airdrop(bob.publicKey, 50);
    await airdrop(eve.publicKey, 50);
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
        {
          policy: findPolicyPk(uuid),
          authority: alice.publicKey,
          uuid,
        },
        { arg: { jsonRule, dynamicRoyalty: null } }
      );
      await process_tx(conn, [ix], [alice]);
      const policy = await Policy.fromAccountAddress(conn, findPolicyPk(uuid));
      assert.isTrue(policy.authority.equals(alice.publicKey));
    });

    it("happy path with dynamic royalty", async () => {
      const uuid = Keypair.generate().publicKey;
      const jsonRule = JSON.stringify({
        events: [],
        conditions: {
          and: [{ field: "action", operator: "string_not_equals", value: "" }],
        },
      });
      const dynamicRoyalty = createDynamicRoyaltyStruct({
        startPrice: new anchor.BN(LAMPORTS_PER_SOL),
        endPrice: new anchor.BN(LAMPORTS_PER_SOL * 2),
        startMultiplierBp: 10000,
        endMultiplierBp: 5000,
      });
      const ix = createInitPolicyInstruction(
        {
          policy: findPolicyPk(uuid),
          authority: alice.publicKey,
          uuid,
        },
        { arg: { jsonRule, dynamicRoyalty } }
      );
      await process_tx(conn, [ix], [alice]);
      const policy = await Policy.fromAccountAddress(conn, findPolicyPk(uuid));
      assert.isTrue(policy.authority.equals(alice.publicKey));
    });

    it("big payload ok", async () => {
      // about 12 "AND" rules before hitting memory limit
      const jsonRule = JSON.stringify({
        events: [],
        conditions: {
          and: Array(12).fill({
            field: "action",
            operator: "string_not_equals",
            value: "",
          }),
        },
      });
      const uuid = Keypair.generate().publicKey;
      const ix = createInitPolicyInstruction(
        {
          policy: findPolicyPk(uuid),
          authority: alice.publicKey,
          uuid,
        },
        { arg: { jsonRule, dynamicRoyalty: null } }
      );
      await process_tx(conn, [ix], [alice]);
      const policy = await Policy.fromAccountAddress(conn, findPolicyPk(uuid));
      assert.isTrue(policy.authority.equals(alice.publicKey));
    });

    it("big payload with many pubkeys", async () => {
      // about 18 pubkeys before hitting payload limit
      const jsonRule = JSON.stringify({
        events: [],
        conditions: {
          and: [
            {
              field: "program_ids",
              operator: "string_is_subset",
              value: Array(18).fill(Keypair.generate().publicKey),
            },
          ],
        },
      });
      const uuid = Keypair.generate().publicKey;
      const ix = createInitPolicyInstruction(
        {
          policy: findPolicyPk(uuid),
          authority: alice.publicKey,
          uuid,
        },
        { arg: { jsonRule, dynamicRoyalty: null } }
      );
      await process_tx(conn, [ix], [alice]);
      const policy = await Policy.fromAccountAddress(conn, findPolicyPk(uuid));
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
        { arg: { authority: bob.publicKey, jsonRule, dynamicRoyalty: null } }
      );
      await process_tx(conn, [ix], [alice]);
      {
        const policy = await Policy.fromAccountAddress(
          conn,
          findPolicyPk(uuid)
        );
        assert.isTrue(policy.authority.equals(bob.publicKey));
      }

      ix = createUpdatePolicyInstruction(
        { policy: findPolicyPk(uuid), authority: bob.publicKey },
        { arg: { authority: alice.publicKey, jsonRule, dynamicRoyalty: null } }
      );
      await process_tx(conn, [ix], [bob]);
      {
        const policy = await Policy.fromAccountAddress(
          conn,
          findPolicyPk(uuid)
        );
        assert.isTrue(policy.authority.equals(alice.publicKey));
      }

      try {
        await process_tx(conn, [ix], [bob]);
        assert.fail("should have failed");
      } catch (e: any) {
        assert.include(e.message, "Transaction simulation failed");
      }
    });
  });

});
