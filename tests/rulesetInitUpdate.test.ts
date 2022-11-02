import { test, beforeAll, expect } from "@jest/globals";
import { CardinalProvider, executeTransaction, getProvider } from "./utils";
import { Keypair, Transaction } from "@solana/web3.js";

import {
  createInitRulesetInstruction,
  findRulesetId,
  Ruleset,
  createUpdateRulesetInstruction,
} from "../sdk";

const RULESET_NAME = `global-${Math.random()}`;
const RULESET_ID = findRulesetId(RULESET_NAME);
let provider: CardinalProvider;

beforeAll(async () => {
  provider = await getProvider();
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

test("Update ruleset", async () => {
  const tx = new Transaction();
  const newAuthority = Keypair.generate();
  tx.add(
    createUpdateRulesetInstruction(
      {
        ruleset: RULESET_ID,
        authority: provider.wallet.publicKey,
      },
      {
        ix: {
          authority: newAuthority.publicKey,
          collector: provider.wallet.publicKey,
          checkSellerFeeBasisPoints: true,
          disallowedAddresses: [provider.wallet.publicKey],
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
  expect(ruleset.authority.toString()).toBe(newAuthority.publicKey.toString());
  expect(ruleset.checkSellerFeeBasisPoints).toBe(true);
  expect(ruleset.disallowedAddresses.length).toBe(1);
  expect(ruleset.allowedPrograms.length).toBe(0);
});
