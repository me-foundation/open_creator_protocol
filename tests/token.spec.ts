import { findMetadataPda } from "@metaplex-foundation/js";
import * as anchor from "@project-serum/anchor";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAccount,
  getAssociatedTokenAddress,
  getMint
} from "@solana/spl-token";
import { Keypair, SYSVAR_INSTRUCTIONS_PUBKEY } from "@solana/web3.js";
import { assert } from "chai";
import {
  CMT_PROGRAM,
  computeBudgetIx,
  createApproveInstruction,
  createBurnInstruction,
  createCloseInstruction,
  createInitAccountInstruction,
  createLockInstruction,
  createMintToInstruction,
  createRevokeInstruction,
  createTransferInstruction,
  createUnlockInstruction,
  findFreezeAuthorityPk,
  findMintStatePk,
  MintState,
  process_tx
} from "../sdk/src";
import {
  airdrop,
  conn,
  createPolicyFixture,
  createTestMintAndWrap,
  DEVNET_POLICY_ALL
} from "./utils";

describe("policy", () => {
  const alice = Keypair.generate();
  const bob = Keypair.generate();
  const eve = Keypair.generate();

  beforeEach(async () => {
    await airdrop(alice.publicKey, 50);
    await airdrop(bob.publicKey, 50);
    await airdrop(eve.publicKey, 50);
  });

  describe("Can wrap a token", () => {
    it("happy path with default policy", async () => {
      const [tokenMint, tokenAta] = await createTestMintAndWrap(
        conn,
        new anchor.Wallet(alice),
        DEVNET_POLICY_ALL
      );
      assert.isNotEmpty(tokenMint.toBase58());
      assert.isNotEmpty(tokenAta.toBase58());
    });

    it("happy path", async () => {
      const [tokenMint, tokenAta] = await createTestMintAndWrap(
        conn,
        new anchor.Wallet(alice),
        await createPolicyFixture(conn, alice)
      );
      assert.isNotEmpty(tokenMint.toBase58());
      assert.isNotEmpty(tokenAta.toBase58());
    });
  });

  describe("Can burn a token", () => {
    it("happy path", async () => {
      const [tokenMint, tokenAta] = await createTestMintAndWrap(
        conn,
        new anchor.Wallet(alice),
        DEVNET_POLICY_ALL
      );
      assert.isNotEmpty(tokenMint.toBase58());
      assert.isNotEmpty(tokenAta.toBase58());

      const ix = createBurnInstruction({
        policy: DEVNET_POLICY_ALL,
        freezeAuthority: findFreezeAuthorityPk(DEVNET_POLICY_ALL),
        mint: tokenMint,
        metadata: findMetadataPda(tokenMint),
        mintState: findMintStatePk(tokenMint),
        from: alice.publicKey,
        fromAccount: tokenAta,
        cmtProgram: CMT_PROGRAM,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      });
      await process_tx(conn, [computeBudgetIx, ix], [alice]);

      try {
        await MintState.fromAccountAddress(conn, tokenMint);
        assert.fail("should have thrown");
      } catch (e: any) {
        assert.include(e.message, "Expected");
      }
      const mint = await getMint(conn, tokenMint);
      assert.equal(mint.supply.toString(), "0");
    });

    it("burn other people's token should fail", async () => {
      const [tokenMint, tokenAta] = await createTestMintAndWrap(
        conn,
        new anchor.Wallet(alice),
        DEVNET_POLICY_ALL
      );
      assert.isNotEmpty(tokenMint.toBase58());
      assert.isNotEmpty(tokenAta.toBase58());

      const ix = createBurnInstruction({
        policy: DEVNET_POLICY_ALL,
        freezeAuthority: findFreezeAuthorityPk(DEVNET_POLICY_ALL),
        mint: tokenMint,
        metadata: findMetadataPda(tokenMint),
        mintState: findMintStatePk(tokenMint),
        from: bob.publicKey,
        fromAccount: tokenAta,
        cmtProgram: CMT_PROGRAM,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      });

      try {
        await process_tx(conn, [computeBudgetIx, ix], [bob]);
        assert.fail("should have thrown");
      } catch (e: any) {
        assert.include(e.message, "failed to send transaction");
      }
      const mint = await getMint(conn, tokenMint);
      assert.equal(mint.supply.toString(), "1");
    });

    it("burn then cannot mint_to", async () => {
      const [tokenMint, tokenAta] = await createTestMintAndWrap(
        conn,
        new anchor.Wallet(alice),
        DEVNET_POLICY_ALL
      );
      assert.isNotEmpty(tokenMint.toBase58());
      assert.isNotEmpty(tokenAta.toBase58());

      const ix = createBurnInstruction({
        policy: DEVNET_POLICY_ALL,
        freezeAuthority: findFreezeAuthorityPk(DEVNET_POLICY_ALL),
        mint: tokenMint,
        metadata: findMetadataPda(tokenMint),
        mintState: findMintStatePk(tokenMint),
        from: alice.publicKey,
        fromAccount: tokenAta,
        cmtProgram: CMT_PROGRAM,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      });

      await process_tx(conn, [computeBudgetIx, ix], [alice]);

      try {
        await MintState.fromAccountAddress(conn, tokenMint);
        assert.fail("should have thrown");
      } catch (e: any) {
        assert.include(e.message, "Expected");
      }
      const mint = await getMint(conn, tokenMint);
      assert.equal(mint.supply.toString(), "0");

      const mintToIx = createMintToInstruction({
        policy: DEVNET_POLICY_ALL,
        freezeAuthority: findFreezeAuthorityPk(DEVNET_POLICY_ALL),
        mint: tokenMint,
        metadata: findMetadataPda(tokenMint),
        mintState: findMintStatePk(tokenMint),
        from: alice.publicKey,
        fromAccount: tokenAta,
        cmtProgram: CMT_PROGRAM,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
        payer: alice.publicKey,
      })
      try {
        await process_tx(conn, [computeBudgetIx, mintToIx], [alice]);
        assert.fail("should have thrown");
      } catch (e: any) {
        assert.include(e.message, "failed to send transaction");
      }

    });
  });

  describe("Can approve a delegate", () => {
    it("happy path", async () => {
      const [tokenMint, aliceAta] = await createTestMintAndWrap(
        conn,
        new anchor.Wallet(alice),
        DEVNET_POLICY_ALL
      );
      assert.isNotEmpty(tokenMint.toBase58());
      assert.isNotEmpty(aliceAta.toBase58());

      const approveIx = createApproveInstruction({
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
      });

      await process_tx(conn, [computeBudgetIx, approveIx], [alice]);

      const mint = await getMint(conn, tokenMint);
      assert.equal(mint.supply.toString(), "1");
      let aliceAtaAcc = await getAccount(conn, aliceAta);
      assert.equal(aliceAtaAcc.amount.toString(), "1");
      assert.equal(aliceAtaAcc.delegate?.toBase58(), bob.publicKey.toBase58());

      const revokeIx = createRevokeInstruction({
        policy: DEVNET_POLICY_ALL,
        freezeAuthority: findFreezeAuthorityPk(DEVNET_POLICY_ALL),
        mint: tokenMint,
        metadata: findMetadataPda(tokenMint),
        mintState: findMintStatePk(tokenMint),
        from: alice.publicKey,
        fromAccount: aliceAta,
        cmtProgram: CMT_PROGRAM,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      });
      await process_tx(conn, [computeBudgetIx, revokeIx], [alice]);
      aliceAtaAcc = await getAccount(conn, aliceAta);
      assert.equal(aliceAtaAcc.amount.toString(), "1");
      assert.equal(aliceAtaAcc.delegate, null);
    });
  });

  describe("Can transfer a token", () => {
    it("happy path", async () => {
      const [tokenMint, aliceAta] = await createTestMintAndWrap(
        conn,
        new anchor.Wallet(alice),
        DEVNET_POLICY_ALL
      );
      assert.isNotEmpty(tokenMint.toBase58());
      assert.isNotEmpty(aliceAta.toBase58());

      const bobAta = await getAssociatedTokenAddress(tokenMint, bob.publicKey);

      const initAccIx = createInitAccountInstruction({
        policy: DEVNET_POLICY_ALL,
        freezeAuthority: findFreezeAuthorityPk(DEVNET_POLICY_ALL),
        mint: tokenMint,
        metadata: findMetadataPda(tokenMint),
        mintState: findMintStatePk(tokenMint),
        from: bob.publicKey,
        fromAccount: bobAta,
        cmtProgram: CMT_PROGRAM,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
        payer: alice.publicKey,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      });

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

      const mint = await getMint(conn, tokenMint);
      assert.equal(mint.supply.toString(), "1");
      const bobAtaAcc = await getAccount(conn, bobAta);
      assert.equal(bobAtaAcc.amount.toString(), "1");
      const aliceAtaAcc = await getAccount(conn, aliceAta);
      assert.equal(aliceAtaAcc.amount.toString(), "0");
    });

    it("happy path with complex policy fixture and close token", async () => {
      const policy = await createPolicyFixture(conn, alice);
      const [tokenMint, aliceAta] = await createTestMintAndWrap(
        conn,
        new anchor.Wallet(alice),
        policy
      );
      assert.isNotEmpty(tokenMint.toBase58());
      assert.isNotEmpty(aliceAta.toBase58());

      const bobAta = await getAssociatedTokenAddress(tokenMint, bob.publicKey);

      const initAccIx = createInitAccountInstruction({
        policy,
        freezeAuthority: findFreezeAuthorityPk(policy),
        mint: tokenMint,
        metadata: findMetadataPda(tokenMint),
        mintState: findMintStatePk(tokenMint),
        from: bob.publicKey,
        fromAccount: bobAta,
        cmtProgram: CMT_PROGRAM,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
        payer: alice.publicKey,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      });

      const transferIx = createTransferInstruction({
        policy,
        freezeAuthority: findFreezeAuthorityPk(policy),
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

      const mint = await getMint(conn, tokenMint);
      assert.equal(mint.supply.toString(), "1");
      const bobAtaAcc = await getAccount(conn, bobAta);
      assert.equal(bobAtaAcc.amount.toString(), "1");
      const aliceAtaAcc = await getAccount(conn, aliceAta);
      assert.equal(aliceAtaAcc.amount.toString(), "0");

      const closeIx = createCloseInstruction({
        policy,
        freezeAuthority: findFreezeAuthorityPk(policy),
        mint: tokenMint,
        metadata: findMetadataPda(tokenMint),
        mintState: findMintStatePk(tokenMint),
        from: alice.publicKey,
        fromAccount: aliceAta,
        cmtProgram: CMT_PROGRAM,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
        destination: alice.publicKey,
      });
      await process_tx(conn, [computeBudgetIx, closeIx], [alice]);
      try {
        await getAccount(conn, aliceAta);
        assert.fail("should fail here");
      } catch (e: any) {
        assert.isNotEmpty(e);
      }
    });

    it("happy path with approved delegate", async () => {
      const [tokenMint, aliceAta] = await createTestMintAndWrap(
        conn,
        new anchor.Wallet(alice),
        DEVNET_POLICY_ALL
      );
      assert.isNotEmpty(tokenMint.toBase58());
      assert.isNotEmpty(aliceAta.toBase58());

      const bobAta = await getAssociatedTokenAddress(tokenMint, bob.publicKey);

      const approveIx = createApproveInstruction({
        policy: DEVNET_POLICY_ALL,
        freezeAuthority: findFreezeAuthorityPk(DEVNET_POLICY_ALL),
        mint: tokenMint,
        metadata: findMetadataPda(tokenMint),
        mintState: findMintStatePk(tokenMint),
        from: alice.publicKey,
        fromAccount: aliceAta,
        cmtProgram: CMT_PROGRAM,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
        to: eve.publicKey,
      });

      const initAccIx = createInitAccountInstruction({
        policy: DEVNET_POLICY_ALL,
        freezeAuthority: findFreezeAuthorityPk(DEVNET_POLICY_ALL),
        mint: tokenMint,
        metadata: findMetadataPda(tokenMint),
        mintState: findMintStatePk(tokenMint),
        from: bob.publicKey,
        fromAccount: bobAta,
        cmtProgram: CMT_PROGRAM,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
        payer: eve.publicKey,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      });

      const transferIx = createTransferInstruction({
        policy: DEVNET_POLICY_ALL,
        freezeAuthority: findFreezeAuthorityPk(DEVNET_POLICY_ALL),
        mint: tokenMint,
        metadata: findMetadataPda(tokenMint),
        mintState: findMintStatePk(tokenMint),
        from: eve.publicKey,
        fromAccount: aliceAta,
        cmtProgram: CMT_PROGRAM,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
        to: bob.publicKey,
        toAccount: bobAta,
      });

      await process_tx(conn, [computeBudgetIx, approveIx], [alice]);
      await process_tx(conn, [computeBudgetIx, initAccIx, transferIx], [eve]);

      const mint = await getMint(conn, tokenMint);
      assert.equal(mint.supply.toString(), "1");
      const bobAtaAcc = await getAccount(conn, bobAta);
      assert.equal(bobAtaAcc.amount.toString(), "1");
      const aliceAtaAcc = await getAccount(conn, aliceAta);
      assert.equal(aliceAtaAcc.amount.toString(), "0");
    });

    it("happy path with approved and locked delegate", async () => {
      const [tokenMint, aliceAta] = await createTestMintAndWrap(
        conn,
        new anchor.Wallet(alice),
        DEVNET_POLICY_ALL
      );
      assert.isNotEmpty(tokenMint.toBase58());
      assert.isNotEmpty(aliceAta.toBase58());

      const bobAta = await getAssociatedTokenAddress(tokenMint, bob.publicKey);

      const approveIx = createApproveInstruction({
        policy: DEVNET_POLICY_ALL,
        freezeAuthority: findFreezeAuthorityPk(DEVNET_POLICY_ALL),
        mint: tokenMint,
        metadata: findMetadataPda(tokenMint),
        mintState: findMintStatePk(tokenMint),
        from: alice.publicKey,
        fromAccount: aliceAta,
        cmtProgram: CMT_PROGRAM,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
        to: eve.publicKey,
      });

      const lockIx = createLockInstruction({
        policy: DEVNET_POLICY_ALL,
        mint: tokenMint,
        metadata: findMetadataPda(tokenMint),
        mintState: findMintStatePk(tokenMint),
        from: alice.publicKey,
        fromAccount: aliceAta,
        cmtProgram: CMT_PROGRAM,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
        to: eve.publicKey,
      });

      const unlockIx = createUnlockInstruction({
        policy: DEVNET_POLICY_ALL,
        mint: tokenMint,
        metadata: findMetadataPda(tokenMint),
        mintState: findMintStatePk(tokenMint),
        from: eve.publicKey,
        cmtProgram: CMT_PROGRAM,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      });

      const initAccIx = createInitAccountInstruction({
        policy: DEVNET_POLICY_ALL,
        freezeAuthority: findFreezeAuthorityPk(DEVNET_POLICY_ALL),
        mint: tokenMint,
        metadata: findMetadataPda(tokenMint),
        mintState: findMintStatePk(tokenMint),
        from: bob.publicKey,
        fromAccount: bobAta,
        cmtProgram: CMT_PROGRAM,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
        payer: eve.publicKey,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      });

      const transferIx = createTransferInstruction({
        policy: DEVNET_POLICY_ALL,
        freezeAuthority: findFreezeAuthorityPk(DEVNET_POLICY_ALL),
        mint: tokenMint,
        metadata: findMetadataPda(tokenMint),
        mintState: findMintStatePk(tokenMint),
        from: eve.publicKey,
        fromAccount: aliceAta,
        cmtProgram: CMT_PROGRAM,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
        to: bob.publicKey,
        toAccount: bobAta,
      });

      await process_tx(conn, [computeBudgetIx, approveIx, lockIx], [alice]);
      await process_tx(
        conn,
        [computeBudgetIx, unlockIx, initAccIx, transferIx],
        [eve]
      );

      const mint = await getMint(conn, tokenMint);
      assert.equal(mint.supply.toString(), "1");
      const bobAtaAcc = await getAccount(conn, bobAta);
      assert.equal(bobAtaAcc.amount.toString(), "1");
      const aliceAtaAcc = await getAccount(conn, aliceAta);
      assert.equal(aliceAtaAcc.amount.toString(), "0");
    });
  });
});
