import { test, expect } from "@jest/globals";
import { CardinalProvider, executeTransaction, getProvider } from "./utils";
import { Keypair, Transaction } from "@solana/web3.js";

import {
  findMintManagerId,
  MintManager,
  createInitRulesetInstruction,
  findRulesetId,
  Ruleset,
  createInitMintInstruction,
} from "../sdk";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";

const RULESET_NAME = `global-${Math.random()}`;
const RULESET_ID = findRulesetId(RULESET_NAME);
let provider: CardinalProvider;

test("Create ruleset", async () => {
  provider = await getProvider();
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

test("Init", async () => {
  const mintKeypair = Keypair.generate();
  const mint = mintKeypair.publicKey;
  const mintManagerId = findMintManagerId(mint);
  const ruleset = await Ruleset.fromAccountAddress(
    provider.connection,
    RULESET_ID
  );

  const tx = new Transaction();
  tx.add(
    createInitMintInstruction({
      mintManager: mintManagerId,
      mint: mint,
      ruleset: RULESET_ID,
      targetTokenAccount: getAssociatedTokenAddressSync(
        mintKeypair.publicKey,
        provider.wallet.publicKey
      ),
      target: provider.wallet.publicKey,
      authority: provider.wallet.publicKey,
      payer: provider.wallet.publicKey,
      collector: ruleset.collector,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
  );
  await executeTransaction(provider.connection, tx, provider.wallet, [
    mintKeypair,
  ]);

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
