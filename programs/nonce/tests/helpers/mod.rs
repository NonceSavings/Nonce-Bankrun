pub mod utils;
use {
    anchor_lang::error::ERROR_CODE_OFFSET,
    nonce::state::{SavingsAccount, SavingsType},
    solana_program_test::{BanksClient, BanksClientError, ProgramTestContext},
    solana_sdk::{
        clock::Clock,
        instruction::{Instruction, InstructionError},
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
        system_instruction::transfer,
        system_program,
        transaction::{Transaction, TransactionError},
    },
    spl_associated_token_account::get_associated_token_address_with_program_id,
};

#[allow(dead_code)]
pub async fn airdrop(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    receiver: &Pubkey,
    amount: u64,
) -> Result<(), BanksClientError> {
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&payer.pubkey(), receiver, amount)],
        Some(&payer.pubkey()),
        &[payer],
        banks_client.get_latest_blockhash().await?,
    );

    banks_client.process_transaction(transaction).await
}

#[allow(dead_code)]
pub fn initialize(
    program_id: Pubkey,
    token_program_id: Pubkey,
    signer: Pubkey,
    mint: Pubkey,
    name: String,
    description: String,
    amount: u64,
    is_sol: bool,
    savings_type: SavingsType,
    lock_duration: Option<i64>,
    unlock_price: Option<u64>,
) -> Instruction {
    let (savings_account, _) = Pubkey::find_program_address(
        &[name.as_bytes(), signer.as_ref(), description.as_bytes()],
        &program_id,
    );
    let (protocol_account, _) = Pubkey::find_program_address(&[b"protocol", signer.as_ref()], &program_id);

    let vault =
        get_associated_token_address_with_program_id(&savings_account, &mint, &token_program_id);
    Instruction {
        program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &nonce::accounts::InitializeSavings {
                signer,
                mint,
                protocol_account,
                savings_account,
                token_vault_account: vault,
                token_program: token_program_id,
                system_program: system_program::id(),
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&nonce::instruction::InitializeSavings {
            name,
            description,
            savings_type,
            is_sol,
            amount,
            lock_duration,
            unlock_price,
        }),
    }
}
