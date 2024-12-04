use std::fmt::Error;
use anchor_lang::prelude::ProgramError;
use anchor_spl::token_2022::{
    MintTo,
    mint_to
};
pub use sha256::digest;

pub fn validate_input(sig: String) -> Result<(), ProgramError> {

    let hash = digest(sig);

    if (hash == "c0754dd78e6e35d7b10126a84772c7aabd31e1ab08652581694f5a328e6f19bc"){
        
    }

    Ok(())
}