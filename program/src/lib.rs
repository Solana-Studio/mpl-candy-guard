use anchor_lang::prelude::*;

use instructions::*;

pub mod errors;
pub mod guards;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("FhCHXHuD6r2iCGwHgqcgnDbwXprLf22pZcArSp4Si4n7");

#[program]
pub mod candy_guard {
    use super::*;

    /// Create a new candy guard account.
    pub fn initialize(ctx: Context<Initialize>, data: Vec<u8>) -> Result<()> {
        instructions::initialize(ctx, data)
    }

    /// Mint an NFT from a candy machine wrapped in the candy guard.
    pub fn mint<'info>(
        ctx: Context<'_, '_, '_, 'info, Mint<'info>>,
        mint_args: Vec<u8>,
        label: Option<String>,
    ) -> Result<()> {
        instructions::mint(ctx, mint_args, label)
    }

    /// Route the transaction to a guard instruction.
    pub fn route<'info>(
        ctx: Context<'_, '_, '_, 'info, Route<'info>>,
        args: RouteArgs,
        label: Option<String>,
    ) -> Result<()> {
        instructions::route(ctx, args, label)
    }

    /// Set a new authority of the candy guard.
    pub fn set_authority(ctx: Context<SetAuthority>, new_authority: Pubkey) -> Result<()> {
        instructions::set_authority(ctx, new_authority)
    }

    /// Update the candy guard configuration.
    pub fn update(ctx: Context<Update>, data: Vec<u8>) -> Result<()> {
        instructions::update(ctx, data)
    }

    /// Withdraw the rent SOL from the candy guard account.
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        instructions::withdraw(ctx)
    }
}
