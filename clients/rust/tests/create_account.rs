#![cfg(feature = "test-sbf")]

use ephemeris_client::{find_pda, instructions::CreateAccountBuilder};
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{
    clock::Clock,
    instruction::InstructionError,
    signature::{Keypair, Signer},
    system_program,
    transaction::Transaction,
};

macro_rules! assert_instruction_error {
    ( $error:expr, $matcher:pat ) => {
        match $error {
            solana_program_test::BanksClientError::TransactionError(
                solana_sdk::transaction::TransactionError::InstructionError(_, $matcher),
            ) => {
                assert!(true)
            }
            err => assert!(false, "Expected instruction error but got '{:#?}'", err),
        };
    };
}

#[tokio::test]
async fn create_account() {
    let mut context = ProgramTest::new("ephemeris_program", ephemeris_client::ID, None)
        .start_with_context()
        .await;

    // Given a PDA derived from a base pubkey and the current slot.

    let base = Keypair::new();
    let slot = context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .unwrap()
        .slot;
    let (pda, _) = find_pda(&base.pubkey(), slot);

    // When we create an account for the PDA.

    let ix = CreateAccountBuilder::new()
        .base(Some(base.pubkey()))
        .from(context.payer.pubkey())
        .to(pda)
        .slot(slot)
        .lamports(5_000_000_000)
        .space(200)
        .owner(system_program::ID)
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &base],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    // Then an account was created with the correct data.

    let account = context.banks_client.get_account(pda).await.unwrap();

    assert!(account.is_some());

    let account = account.unwrap();
    assert_eq!(account.lamports, 5_000_000_000);
    assert_eq!(account.data.len(), 200);
    assert_eq!(account.owner, system_program::ID);
}

#[tokio::test]
async fn fail_create_account_with_old_slot() {
    let mut context = ProgramTest::new("ephemeris_program", ephemeris_client::ID, None)
        .start_with_context()
        .await;

    // Given a PDA derived from a base pubkey and the current slot.

    let base = Keypair::new();
    let slot = context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .unwrap()
        .slot;
    let (pda, _) = find_pda(&base.pubkey(), slot);

    // And the slot is then incremented.

    context.warp_to_slot(slot + 200).unwrap();

    // When we try to create an account for the PDA with an old slot.

    let ix = CreateAccountBuilder::new()
        .base(Some(base.pubkey()))
        .from(context.payer.pubkey())
        .to(pda)
        .slot(slot)
        .lamports(5_000_000_000)
        .space(200)
        .owner(system_program::ID)
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &base],
        context.last_blockhash,
    );
    let err = context
        .banks_client
        .process_transaction(tx)
        .await
        .unwrap_err();

    // Then we expect an error.

    assert_instruction_error!(err, InstructionError::InvalidArgument);
}

#[tokio::test]
async fn fail_create_account_with_invalid_derivation() {
    let mut context = ProgramTest::new("ephemeris_program", ephemeris_client::ID, None)
        .start_with_context()
        .await;

    // Given a PDA derived from a base pubkey and the current slot.

    let base = Keypair::new();
    let slot = context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .unwrap()
        .slot;

    // And we derive a PDA with a different slot.

    let (pda, _) = find_pda(&base.pubkey(), slot + 100);

    // When we try to create an account for the PDA.

    let ix = CreateAccountBuilder::new()
        .base(Some(base.pubkey()))
        .from(context.payer.pubkey())
        .to(pda)
        .slot(slot)
        .lamports(5_000_000_000)
        .space(200)
        .owner(system_program::ID)
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &base],
        context.last_blockhash,
    );
    let err = context
        .banks_client
        .process_transaction(tx)
        .await
        .unwrap_err();

    // Then we expect an error.

    assert_instruction_error!(err, InstructionError::InvalidSeeds);
}
