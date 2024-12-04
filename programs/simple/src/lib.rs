use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken,  token_interface::{
    Mint, TokenAccount, TokenInterface
}};
declare_id!("DEFuzL6ArEcszLSgy1pQBLSdyBd7BKR5CUdckq2RXn2A");

pub mod states;
pub mod instructions;
pub mod validation_function;


use states::game_state::config::Config;
use instructions::{
    game_instructions::init_config::*,
    game_instructions::init_user::*,
    vuln_instructions::create_user::*,
    vuln_instructions::transfer_points::*
};

pub use crate::validation_function::validate_input::validate_inputs;

#[program]
pub mod simple {
    use super::*;
    pub fn initialize_config(ctx: Context<InitConfig>) -> Result<()> {
        ctx.accounts.config_init(&ctx.bumps)
    }

    pub fn initialize_user(ctx: Context<InitUser>) -> Result<()> {
        ctx.accounts.init_user()
    }

    pub fn user_create(ctx: Context<CreateUser>, name: String) -> Result<()> {
        ctx.accounts.init_user(name)
    }

    pub fn transfer_point(ctx: Context<TransferPoints>, _id_sender: u32, _id_receiver: u32, amount: u16) -> Result<()> {
        ctx.accounts.transfer_points(_id_sender, _id_receiver, amount)
    }

    pub fn space_validation(ctx: Context<PassTest>, answer: String) -> Result<()> {
        validate_inputs(
            answer, 
            "c0754dd78e6e35d7b10126a84772c7aabd31e1ab08652581694f5a328e6f19bc", 
            ctx.accounts.user_ata.to_account_info(), 
            ctx.accounts.config.to_account_info(), 
            ctx.accounts.mint_account.to_account_info(),
            ctx.accounts.token_program.to_account_info()
        );
        Ok(())
    }

    

}


#[derive(Accounts)]
pub struct PassTest<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // The config and it's seeds 
    pub config: Account<'info, Config>,
    // The platform's mint
    pub mint_account: InterfaceAccount<'info, Mint>,
    // The user pda with the mint
    #[account(
        init_if_needed,
        payer= signer, 
        associated_token::mint = mint_account,
        associated_token::authority = signer
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>
}