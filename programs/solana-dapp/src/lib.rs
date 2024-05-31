use anchor_lang::prelude::*;

declare_id!("5nrPYHucWFU2kbaVtzDyuRDmny4McbNwxDTqQSYrFV9s");

#[program]
pub mod solana_dapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
