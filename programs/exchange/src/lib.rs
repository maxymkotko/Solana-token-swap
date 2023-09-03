use anchor_lang::prelude::*;
mod constants;
mod curve;
mod errors;
mod instructions;
mod state;
mod utils;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod exchange {
    use super::*;

    pub fn initialize(ctx: Context<InitializePool>, fees: Fee) -> Result<()> {
        ctx.accounts.initialize(fees)
    }
}
