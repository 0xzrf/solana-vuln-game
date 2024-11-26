use anchor_lang::prelude::*;

declare_id!("4busdpFhfoenYZbhmtj6YHYyKq9cvoGqBUTimp5URdeW");

#[program]
pub mod solana_vuln_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
