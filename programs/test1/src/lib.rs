use anchor_lang::prelude::*;

declare_id!("F51kSctUgWPRt2pRKoXZ6txLJwRfvkBb3tkxFtVzRv8x");

#[program]
pub mod test1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
