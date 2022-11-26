use solana_program::program_option::COption;
use solana_program_test::*;
use solana_sdk::{
    commitment_config::CommitmentLevel,
    instruction::Instruction,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::Signature,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

use mtoken::instruction::*;
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token::state::Account as TokenAccount;

pub fn sol(amount: f64) -> u64 {
    (amount * LAMPORTS_PER_SOL as f64) as u64
}

async fn process_transaction(
    client: &mut BanksClient,
    instructions: Vec<Instruction>,
    signers: Vec<&Keypair>,
) -> anyhow::Result<Signature> {
    let mut tx = Transaction::new_with_payer(&instructions, Some(&signers[0].pubkey()));
    tx.partial_sign(&signers, client.get_latest_blockhash().await?);
    let sig = tx.signatures[0];
    client
        .process_transaction_with_commitment(tx, CommitmentLevel::Confirmed)
        .await?;
    Ok(sig)
}

async fn transfer(
    context: &mut BanksClient,
    payer: &Keypair,
    receiver: &Pubkey,
    amount: u64,
) -> anyhow::Result<Signature> {
    let ixs = vec![system_instruction::transfer(
        &payer.pubkey(),
        receiver,
        amount,
    )];
    process_transaction(context, ixs, vec![payer]).await
}

fn spl_managed_token_test() -> ProgramTest {
    ProgramTest::new(
        "spl_managed_token",
        community_managed_token::id(),
        processor!(community_managed_token::process_instruction),
    )
}

#[tokio::test]
async fn test_create_policy() {
    let mut context = spl_managed_token_test().start_with_context().await;
    let lwc = &mut context.banks_client;
    let authority = Keypair::new();
    transfer(lwc, &context.payer, &authority.pubkey(), sol(10.0))
        .await
        .unwrap();
}
