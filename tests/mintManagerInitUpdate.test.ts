import { test, beforeAll, expect } from "@jest/globals";
import { CardinalProvider, executeTransaction, getProvider } from "./utils";
import { PublicKey } from "@solana/web3.js";
import { Keypair, Transaction } from "@solana/web3.js";

import {
  findMintManagerId,
  createInitMintManagerInstruction,
  MintManager,
  createInitRulesetInstruction,
  findRulesetId,
  Ruleset,
  createUpdateMintManagerInstruction,
} from "../sdk";
import { createMintTx } from "./mint";
let mint: PublicKey;

const RULESET_NAME = `global-${Math.random()}`;
const RULESET_NAME2 = `global-${Math.random()}`;
const RULESET_ID = findRulesetId(RULESET_NAME);
let provider: CardinalProvider;

beforeAll(async () => {
  provider = await getProvider();
  const mintKeypair = Keypair.generate();
  mint = mintKeypair.publicKey;
  executeTransaction(
    provider.connection,
    await createMintTx(provider.connection, mint, provider.wallet.publicKey),
    provider.wallet,
    [mintKeypair]
  );
});

test("Create ruleset", async () => {
  const tx = new Transaction();
  tx.add(
    createInitRulesetInstruction(
      {
        ruleset: RULESET_ID,
        authority: provider.wallet.publicKey,
        payer: provider.wallet.publicKey,
      },
      {
        ix: {
          name: RULESET_NAME,
          collector: provider.wallet.publicKey,
          checkSellerFeeBasisPoints: true,
          disallowedAddresses: [],
          allowedPrograms: [],
        },
      }
    )
  );
  await executeTransaction(provider.connection, tx, provider.wallet);
  const ruleset = await Ruleset.fromAccountAddress(
    provider.connection,
    RULESET_ID
  );
  expect(ruleset.authority.toString()).toBe(
    provider.wallet.publicKey.toString()
  );
  expect(ruleset.checkSellerFeeBasisPoints).toBe(true);
  expect(ruleset.disallowedAddresses.length).toBe(0);
  expect(ruleset.allowedPrograms.length).toBe(0);
});

test("Create 2nd ruleset", async () => {
  const tx = new Transaction();
  tx.add(
    createInitRulesetInstruction(
      {
        ruleset: findRulesetId(RULESET_NAME2),
        authority: provider.wallet.publicKey,
        payer: provider.wallet.publicKey,
      },
      {
        ix: {
          name: RULESET_NAME2,
          collector: provider.wallet.publicKey,
          checkSellerFeeBasisPoints: false,
          disallowedAddresses: [provider.wallet.publicKey],
          allowedPrograms: [],
        },
      }
    )
  );
  await executeTransaction(provider.connection, tx, provider.wallet);
  const ruleset = await Ruleset.fromAccountAddress(
    provider.connection,
    findRulesetId(RULESET_NAME2)
  );
  expect(ruleset.authority.toString()).toBe(
    provider.wallet.publicKey.toString()
  );
  expect(ruleset.checkSellerFeeBasisPoints).toBe(false);
  expect(ruleset.disallowedAddresses.length).toBe(1);
  expect(ruleset.allowedPrograms.length).toBe(0);
});

test("Init mint manager", async () => {
  const mintManagerId = findMintManagerId(mint);
  const tx = new Transaction();
  const ruleset = await Ruleset.fromAccountAddress(
    provider.connection,
    RULESET_ID
  );

  tx.add(
    createInitMintManagerInstruction({
      mint: mint,
      mintManager: mintManagerId,
      authority: provider.wallet.publicKey,
      payer: provider.wallet.publicKey,
      collector: ruleset.collector,
      ruleset: RULESET_ID,
    })
  );
  await executeTransaction(provider.connection, tx, provider.wallet);

  const mintManager = await MintManager.fromAccountAddress(
    provider.connection,
    mintManagerId
  );
  expect(mintManager.mint.toString()).toBe(mint.toString());
  expect(mintManager.authority.toString()).toBe(
    provider.wallet.publicKey.toString()
  );
  expect(mintManager.ruleset.toString()).toBe(
    findRulesetId(RULESET_NAME).toString()
  );
});

test("Update mint manager", async () => {
  const newAuthority = Keypair.generate();
  const mintManagerId = findMintManagerId(mint);
  const tx = new Transaction();
  const ruleset = await Ruleset.fromAccountAddress(
    provider.connection,
    RULESET_ID
  );

  tx.add(
    createUpdateMintManagerInstruction(
      {
        mintManager: mintManagerId,
        authority: provider.wallet.publicKey,
        payer: provider.wallet.publicKey,
        collector: ruleset.collector,
        ruleset: findRulesetId(RULESET_NAME2),
      },
      { ix: { authority: newAuthority.publicKey } }
    )
  );
  await executeTransaction(provider.connection, tx, provider.wallet);

  const mintManager = await MintManager.fromAccountAddress(
    provider.connection,
    mintManagerId
  );
  expect(mintManager.mint.toString()).toBe(mint.toString());
  expect(mintManager.authority.toString()).toBe(
    newAuthority.publicKey.toString()
  );
  expect(mintManager.ruleset.toString()).toBe(
    findRulesetId(RULESET_NAME2).toString()
  );
});
