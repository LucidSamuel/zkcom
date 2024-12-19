use anchor_lang::{prelude::*, solana_program::system_program};
use spl_memo_compressed::{program::SplMemoCompressed, CreateMemo};
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer};

#[tokio::test]
async fn test_create_memo() {
    // Initialize program test
    let program_test = ProgramTest::new(
        "spl_memo_compressed",
        spl_memo_compressed::ID,
        processor!(spl_memo_compressed::entry),
    );

    let mut context = program_test.start_with_context().await;
    let payer = context.payer.insecure_clone();

    // Creating test memo (simulating compressed data from client)
    let test_memo = vec![1, 2, 3, 4]; // Simulated compressed data
    
    let ix = spl_memo_compressed::instruction::CreateMemo {
        compressed_memo: test_memo.clone(),
    };

    let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[ix.instruction()],
        Some(&payer.pubkey()),
        &[&payer],
        context.last_blockhash,
    );

    // Process transaction
    let result = context.banks_client.process_transaction(transaction).await;
    assert!(result.is_ok());

    // Add test for error case - empty memo
    let empty_memo = vec![];
    let ix_empty = spl_memo_compressed::instruction::CreateMemo {
        compressed_memo: empty_memo,
    };

    let transaction_empty = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[ix_empty.instruction()],
        Some(&payer.pubkey()),
        &[&payer],
        context.last_blockhash,
    );

    let result_empty = context.banks_client.process_transaction(transaction_empty).await;
    assert!(result_empty.is_err());
}