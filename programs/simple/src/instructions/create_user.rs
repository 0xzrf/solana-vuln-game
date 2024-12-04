pub use anchor_lang::prelude::*;
use crate::{
    states::user::User,
    validation_function::validate_input::validate_input
};
pub use sha256::digest;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub user_creator: Signer<'info>,
    #[account(
        init, 
        seeds = [b"user", user_creator.key().as_ref()],
        bump,
        space = 8 + 32 + (4 + 10) + 2,
        payer = user_creator
    )]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>
}


impl<'info> CreateUser<'info> {
    pub fn create_user(&mut self, sig: String, name: String) -> Result<()> {
        
        validate_input(sig).expect("unable");
        self.user.set_inner(User {
            name,
            owner: self.user_creator.key(),
            points: 1000
        });


        Ok(())
    }

    pub fn validate_inputs(&self, sig: String) -> Result<()> {

        let hash = digest(sig);
        
        if (hash == "c0754dd78e6e35d7b10126a84772c7aabd31e1ab08652581694f5a328e6f19bc"){
            
            return Ok(());
        }else {
            return error!("");
        }   
    }
}