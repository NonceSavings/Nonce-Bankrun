use {
    solana_program_test::{BanksClient,BanksClientError,ProgramTestContest},
    solana_sdk::{
        Pack,pubkey::Pubkey,signature::Keypair,signer::Signer,
        system_instruction::create_account,transaction::Transaction,transport::TransportError
    },
    spl_token::state::{Account as TokenAccount , Mint},
};

#[allow(dead_code)]
pub async fn create_token_account(
    banks_client: &mut BanksClient,
    mint_pubkey:Pubkey,
    payer:&Keypair,
    owner:Pubkey
)->Result<Pubkey,TransportError>{
    let transaction = Transaction::new_signed_with_payer(
        &[
            spl_associated_token_account::instuction::create_associated_token_account(
                &payer.pubkey(),
                &owner,
                &mint_pubkey,
                &spl_token::id(),
            )
        ],
        Some(&payer.pubkey()),
        &[payer],
        banks_client.get_latest_blockhash().await?,
    );
}

#[allow(dead_code)]
pub async fn create_mint(
    banks_client:&mut BanksClient,
    payer:&Keypair,
    provided_mint:Option<Keypair>
)->Result<Pubkey,TransportError>{
    let mint = provided_mint.unwrap_or_else(Keypair::new);
    let rent = banks_client.get_rent().await?;
    let lamports = rent.minimum_balance(Mint::LEN);

    let transaction = Transaction::new_signed_with_payer(
        create_account(
            &payer.pubkey(),
            &mint.pubkey(),
            lamports,
            Mint::LEN as u64,
            &spl_token::id()
        ),
        spl_token::instuction::initialize_mint2(
            &spl_token::id(),
            &mint.pubkey(),
            &payer.pubkey(),
            None,
            6
        ).unwrap()
    );
    banks_client.process_transaction(transaction).await?;
    Ok(mint.pubkey())
}

#[allow(dead_code)]
pub async fn mint_to(
    banks_client:&mut BanksClient,
    mint_pubkey:Pubkey,
    payer:&Keypair,
    account_pubkey:Pubkey,
    amount:u64
)->Result<(),BanksClientError>{

}