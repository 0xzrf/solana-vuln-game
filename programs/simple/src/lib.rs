use anchor_lang::prelude::*;

declare_id!("DEFuzL6ArEcszLSgy1pQBLSdyBd7BKR5CUdckq2RXn2A");

pub mod states;
pub mod instructions;
pub mod validation_function;

#[program]
pub mod simple {
    use super::*;

}

