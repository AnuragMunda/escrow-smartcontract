use anchor_lang::prelude::*;
use crate::instructions::initialise::*;

pub mod instructions;
pub mod states;

declare_id!("87jE6YRnbAfJ3dgrwxuwQf9kZWkQAQbFGLGcjeRnfmE1");

#[program]
pub mod escrow_contract {
    use super::*;

    pub fn initialise_escrow(
        ctx: Context<Initialise>,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<()> {
        initialise(ctx, amount_a, amount_b)
    }
}
