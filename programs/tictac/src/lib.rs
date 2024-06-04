use anchor_lang::{accounts::account, prelude::*, solana_program::clock};
use std::{collections::HashMap, iter::Map};

declare_id!("5nrPYHucWFU2kbaVtzDyuRDmny4McbNwxDTqQSYrFV9s");

#[program]
pub mod tictac {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     // init value

    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
}
