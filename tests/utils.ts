import {
  findMetadataPda,
  Metaplex,
  walletAdapterIdentity,
} from "@metaplex-foundation/js";
import {
  createCreateMetadataAccountV3Instruction,
  DataV2,
} from "@metaplex-foundation/mpl-token-metadata";
import * as anchor from "@project-serum/anchor";
import {
  AccountLayout,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createInitializeMintInstruction,
  getAssociatedTokenAddress,
  getMinimumBalanceForRentExemptMint,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  ComputeBudgetProgram,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  sendAndConfirmRawTransaction,
  SystemProgram,
  SYSVAR_INSTRUCTIONS_PUBKEY,
  Transaction,
} from "@solana/web3.js";
import {
  CMT_PROGRAM,
  createDynamicRoyaltyStruct,
  createInitAccountInstruction,
  createInitPolicyInstruction,
  createMintToInstruction as ocpCreateMintToInstruction,
  createWrapInstruction,
  findFreezeAuthorityPk,
  findMintStatePk,
  findPolicyPk,
  LARGER_COMPUTE_UNIT,
  process_tx,
} from "../sdk/src";

export const conn = new anchor.AnchorProvider(
  anchor.AnchorProvider.env().connection,
  new anchor.Wallet(Keypair.generate()),
  { commitment: "confirmed" }
).connection;

export const airdrop = async (to: PublicKey, amount: number) => {
  await conn.confirmTransaction({
    ...(await conn.getLatestBlockhash()),
    signature: await conn.requestAirdrop(to, amount * LAMPORTS_PER_SOL),
  });
};

export const DEVNET_POLICY_ALL = new PublicKey(
  "6Huqrb4xxmmNA4NufYdgpmspoLmjXFd3qEfteCddLgSz"
);

export async function createTestMintAndWrap(
  connection: Connection,
  wallet: anchor.Wallet,
  policy = DEVNET_POLICY_ALL
): Promise<[PublicKey, PublicKey]> {
  const metaplex = new Metaplex(connection);
  metaplex.use(walletAdapterIdentity(wallet));

  const mintKeypair = new Keypair();
  const targetTokenAccount = await getAssociatedTokenAddress(
    mintKeypair.publicKey,
    wallet.publicKey
  );

  const tx: Transaction = await createNewMintTransaction(
    connection,
    wallet.payer,
    mintKeypair,
    wallet.publicKey,
    wallet.publicKey
  );
  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
  tx.feePayer = wallet.publicKey;
  tx.add(
    ComputeBudgetProgram.setComputeUnitLimit({ units: LARGER_COMPUTE_UNIT }),
    createWrapInstruction({
      mint: mintKeypair.publicKey,
      policy,
      freezeAuthority: wallet.publicKey,
      mintAuthority: wallet.publicKey,
      mintState: findMintStatePk(mintKeypair.publicKey),
      from: wallet.payer.publicKey,
      instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      cmtProgram: CMT_PROGRAM,
      metadata: findMetadataPda(mintKeypair.publicKey),
    }),
    createInitAccountInstruction({
      policy,
      freezeAuthority: findFreezeAuthorityPk(policy),
      mint: mintKeypair.publicKey,
      metadata: findMetadataPda(mintKeypair.publicKey),
      mintState: findMintStatePk(mintKeypair.publicKey),
      from: wallet.publicKey,
      fromAccount: targetTokenAccount,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      cmtProgram: CMT_PROGRAM,
      payer: wallet.publicKey,
    }),
    ocpCreateMintToInstruction({
      policy,
      freezeAuthority: findFreezeAuthorityPk(policy),
      mint: mintKeypair.publicKey,
      metadata: findMetadataPda(mintKeypair.publicKey),
      mintState: findMintStatePk(mintKeypair.publicKey),
      from: wallet.publicKey,
      fromAccount: targetTokenAccount,
      instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      cmtProgram: CMT_PROGRAM,
      payer: wallet.publicKey,
    })
  );
  tx.partialSign(mintKeypair);
  await wallet.signTransaction(tx);
  try {
    const sig = await sendAndConfirmRawTransaction(connection, tx.serialize());
    console.log({ sig });
  } catch (e) {
    console.error(e);
  }

  return [mintKeypair.publicKey, targetTokenAccount];
}

const createNewMintTransaction = async (
  connection: Connection,
  payer: Keypair,
  mintKeypair: Keypair,
  mintAuthority: PublicKey,
  freezeAuthority: PublicKey
) => {
  //Get the minimum lamport balance to create a new account and avoid rent payments
  const requiredBalance = await getMinimumBalanceForRentExemptMint(connection);
  //metadata account associated with mint
  const metadataPDA = findMetadataPda(mintKeypair.publicKey);

  const ON_CHAIN_METADATA = {
    name: "xyzname",
    symbol: "xyz",
    uri: "example.com",
    sellerFeeBasisPoints: 500,
    creators: [
      { address: Keypair.generate().publicKey, verified: false, share: 100 },
    ],
    collection: null,
    uses: null,
  } as DataV2;

  const createNewTokenTransaction = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: mintKeypair.publicKey,
      space: MINT_SIZE,
      lamports: requiredBalance,
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeMintInstruction(
      mintKeypair.publicKey, //Mint Address
      0, //Number of Decimals of New mint
      mintAuthority, //Mint Authority
      freezeAuthority, //Freeze Authority
      TOKEN_PROGRAM_ID
    ),
    createCreateMetadataAccountV3Instruction(
      {
        metadata: metadataPDA,
        mint: mintKeypair.publicKey,
        mintAuthority: mintAuthority,
        payer: payer.publicKey,
        updateAuthority: mintAuthority,
      },
      {
        createMetadataAccountArgsV3: {
          data: ON_CHAIN_METADATA,
          isMutable: true,
          collectionDetails: null,
        },
      }
    )
  );

  return createNewTokenTransaction;
};

let tokenAccountRent = 0;
export const getTokenAccountRent = async (conn: Connection) => {
  if (tokenAccountRent) {
    return tokenAccountRent;
  }
  tokenAccountRent = await conn.getMinimumBalanceForRentExemption(
    AccountLayout.span
  );
  return tokenAccountRent;
};

export const createPolicyFixture = async (conn: Connection, payer: Keypair) => {
  const uuid = Keypair.generate().publicKey;
  const policy = findPolicyPk(uuid);
  const jsonRule = JSON.stringify({
    events: [],
    conditions: {
      or: [
        { field: "action", operator: "string_not_equals", value: "transfer" },
        {
          and: [
            {
              field: "program_ids",
              operator: "string_does_not_contain_any",
              value: [
                "aaaa111111111111111111111111",
                "bbbb111111111111111111111111",
                "cccc111111111111111111111111",
              ],
            },
            {
              or: [
                {
                  field: "to",
                  operator: "string_not_equals",
                  value: "11111111111111111111111111111111",
                },
                {
                  field: "metadata/name",
                  operator: "string_has_substring",
                  value: "(winner)",
                },
              ],
            },
          ],
        },
      ],
    },
  });
  const dr = createDynamicRoyaltyStruct({
    startMultiplierBp: 10000,
    endMultiplierBp: 0,
    startPrice: new anchor.BN(0),
    endPrice: new anchor.BN(5 * LAMPORTS_PER_SOL),
  });
  const ix = createInitPolicyInstruction(
    { policy, uuid, authority: payer.publicKey },
    { arg: { jsonRule, dynamicRoyalty: dr } }
  );
  await process_tx(conn, [ix], [payer]);
  return policy;
};
