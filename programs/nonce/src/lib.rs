use anchor_lang::prelude::*;

declare_id!("Bh6dp7WU9okucJRGTS53SDSiTaR8mJ187L7wd9WFwKbT");

#[program]
pub mod nonce {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
