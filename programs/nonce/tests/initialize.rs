mod helpers;
use {
    anchor_lang::AccountDeserialize,
    helpers::{utils::*, *},
    nonce::{
        self,
        state::{SavingsAccount, SavingsType},
    },
    rand::Rng,
    solana_program_test::*,
    solana_sdk::{
        native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer,
        transaction::Transaction,
    },
    std::u64,
};

#[tokio::test]
async fn test_initialize() {
    let mut test = ProgramTest::new("nonce", nonce::id(), None);

    test.set_compute_max_units(100_000);

    let maker = Keypair::new();
    let seed: u64 = rand::thread_rng().gen();

    let (mut banks_client, payer, recent_blockhash) = test.start().await;

    let _ = airdrop(
        &mut banks_client,
        &payer,
        &maker.pubkey(),
        2 * LAMPORTS_PER_SOL,
    )
    .await;

    let mint = create_mint(&mut banks_client, &payer, None).await.unwrap();

    let _ =
        create_and_mint_to_token_account(&mut banks_client, mint, &payer, maker.pubkey(), 100_000)
            .await;

    let mut transaction = Transaction::new_with_payer(
        &[initialize(
            nonce::id(),
            spl_token::id(),
            maker.pubkey(),
            mint,
            "Yearly SavingsAccount".to_string(),
            "Yearly Savings Noni".to_string(),
            270,
            true,
            SavingsType::PriceLockedSavings,
            None,
            Some(1000),
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &maker], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let (savings_pubkey, _) = Pubkey::find_program_address(
        &[b"savings", maker.pubkey().as_ref()],
        &nonce::id()
    );

    let savings = banks_client
        .get_account(savings_pubkey)
        .await
        .unwrap()
        .unwrap();

    let mut account_data = savings.data.as_ref();
    let savings_account = SavingsAccount::try_deserialize(&mut account_data).unwrap();
    println!("{:?}",savings_account);

    assert_eq!(savings_account.is_sol,true);
}
