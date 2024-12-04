mod helpers;
use {
    anchor_lang::AccountDeserialize,
    helpers::{utils::*, *},
    nonce,
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
    let seed :u64= rand::thread_rng().gen();

    let (mut banks_client, payer, recent_blockhash) = test.start().await;

    let _ = airdrop(
        &mut banks_client,
        &payer,
        &maker.pubkey(),
        2 * LAMPORTS_PER_SOL,
    )
    .await;
}
