use std::collections::BTreeMap;

use anchor_lang::{prelude::*, solana_program::sysvar};
use solana_program::instruction::Instruction;

use crate::{
    guards::EvaluationContext,
    state::{CandyGuard, CandyGuardData, GuardSet, DATA_OFFSET, SEED},
};

use assetmanager::structs::Asset;

pub fn mint<'info>(
    ctx: Context<'_, '_, '_, 'info, Mint<'info>>,
    mint_args: Vec<u8>,
    label: Option<String>,
) -> Result<()> {
    let custom_args_len_buf: [u8; 2] = *&mint_args[0..2].try_into().unwrap();
    let custom_args_len = u16::from_le_bytes(custom_args_len_buf);
    let custom_args = Vec::from(&mint_args[2..(2 + custom_args_len as usize)]);
    // let mint_args = Vec::from(&mint_args[(2 + custom_args_len as usize)..]);

    let candy_guard = &ctx.accounts.candy_guard;
    let account_info = &candy_guard.to_account_info();
    let account_data = account_info.data.borrow();
    // loads the active guard set
    let guard_set = match CandyGuardData::active_set(&account_data[DATA_OFFSET..], label) {
        Ok(guard_set) => guard_set,
        Err(error) => {
            // load the default guard set to look for the bot_tax since errors only occur
            // when trying to load guard set groups
            let (default, _) = GuardSet::from_data(&account_data[DATA_OFFSET..])?;
            return process_error(&ctx, &default, error);
        }
    };

    let conditions = guard_set.enabled_conditions();

    // evaluation context for this transaction
    let mut evaluation_context = EvaluationContext {
        account_cursor: 0,
        args_cursor: 2 + custom_args_len as usize,
        indices: BTreeMap::new(),
    };

    // validates the required transaction data

    if let Err(error) = validate(&ctx) {
        return process_error(&ctx, &guard_set, error);
    }
    msg!("errors done, {}", evaluation_context.account_cursor);

    // validates enabled guards (any error at this point is subject to bot tax)

    for condition in &conditions {
        if let Err(error) =
            condition.validate(&ctx, &mint_args, &guard_set, &mut evaluation_context)
        {
            return process_error(&ctx, &guard_set, error);
        }
    }
    msg!("validation done, {}", evaluation_context.account_cursor);
    // after this point, errors might occur, which will cause the transaction to fail
    // no bot tax from this point since the actions must be reverted in case of an error

    for condition in &conditions {
        condition.pre_actions(&ctx, &mint_args, &guard_set, &mut evaluation_context)?;
    }
    msg!("pre actions done, {}", evaluation_context.account_cursor);
    cpi(&ctx, custom_args, &mut evaluation_context)?;
    msg!("cpi done, {}", evaluation_context.account_cursor);
    for condition in &conditions {
        condition.post_actions(&ctx, &mint_args, &guard_set, &mut evaluation_context)?;
    }

    Ok(())
}

// Handles errors + bot tax charge.
fn process_error<'info>(
    ctx: &Context<'_, '_, '_, 'info, Mint<'info>>,
    guard_set: &GuardSet,
    error: Error,
) -> Result<()> {
    if let Some(bot_tax) = &guard_set.bot_tax {
        bot_tax.punish_bots(error, ctx)?;
        Ok(())
    } else {
        Err(error)
    }
}

/// Performs a validation of the transaction before executing the guards.
fn validate<'info>(_ctx: &Context<'_, '_, '_, 'info, Mint<'info>>) -> Result<()> {
    // if !cmp_pubkeys(
    //     &ctx.accounts.collection_mint.key(),
    //     &ctx.accounts.candy_machine.collection_mint,
    // ) {
    //     return err!(CandyGuardError::CollectionKeyMismatch);
    // }
    // if !cmp_pubkeys(
    //     ctx.accounts.collection_metadata.owner,
    //     &mpl_token_metadata::id(),
    // ) {
    //     return err!(CandyGuardError::IncorrectOwner);
    // }

    Ok(())
}

/// Send a mint transaction to the candy machine.
fn cpi<'info>(
    ctx: &Context<'_, '_, '_, 'info, Mint<'info>>,
    data: Vec<u8>,
    _evaluation_context: &mut EvaluationContext,
) -> Result<()> {
    let program_id = ctx.accounts.candy_machine_program.key();
    msg!("program id: {}", program_id);
    let rem_accounts = &ctx.remaining_accounts[_evaluation_context.account_cursor..];
    msg!("rem_accounts: {}", rem_accounts.len());
    // PDA signer for the transaction
    let candy_guard = &ctx.accounts.candy_guard;
    let seeds = [SEED, &candy_guard.base.to_bytes(), &[candy_guard.bump]];
    let signer = &[&seeds[..]];
    msg!("candy_guard: {}", candy_guard.key());

    let mut accounts = vec![];
    rem_accounts.iter().for_each(|account| {
        let is_signer = account.is_signer || candy_guard.key() == account.key.key();
        accounts.push(if account.is_writable {
            AccountMeta::new(account.key.key(), is_signer)
        } else {
            AccountMeta::new_readonly(account.key.key(), is_signer)
        });
    });
    msg!("accounts: {}", accounts.len());
    let mint_ix = Instruction {
        program_id,
        accounts,
        data,
    };
    msg!(
        "mint_ix.data: {}, mint_ix.accounts: {}",
        mint_ix.data.len(),
        mint_ix.accounts.len()
    );

    solana_program::program::invoke_signed(&mint_ix, rem_accounts, signer).map_err(Into::into)
}

// /// Send a mint transaction to the candy machine.
// fn cpi_mint<'info>(ctx: &Context<'_, '_, '_, 'info, Mint<'info>>) -> Result<()> {
//     let candy_guard = &ctx.accounts.candy_guard;
//     // PDA signer for the transaction
//     let seeds = [SEED, &candy_guard.base.to_bytes(), &[candy_guard.bump]];
//     let signer = [&seeds[..]];
//     let candy_machine_program = ctx.accounts.candy_machine_program.to_account_info();

//     // candy machine mint instruction accounts
//     let mint_ix = mpl_candy_machine_core::cpi::accounts::Mint {
//         candy_machine: ctx.accounts.candy_machine.to_account_info(),
//         authority_pda: ctx.accounts.candy_machine_authority_pda.to_account_info(),
//         mint_authority: ctx.accounts.candy_guard.to_account_info(),
//         payer: ctx.accounts.payer.to_account_info(),
//         nft_mint: ctx.accounts.nft_mint.to_account_info(),
//         nft_mint_authority: ctx.accounts.nft_mint_authority.to_account_info(),
//         nft_metadata: ctx.accounts.nft_metadata.to_account_info(),
//         nft_master_edition: ctx.accounts.nft_master_edition.to_account_info(),
//         collection_authority_record: ctx.accounts.collection_authority_record.to_account_info(),
//         collection_mint: ctx.accounts.collection_mint.to_account_info(),
//         collection_metadata: ctx.accounts.collection_metadata.to_account_info(),
//         collection_master_edition: ctx.accounts.collection_master_edition.to_account_info(),
//         collection_update_authority: ctx.accounts.collection_update_authority.to_account_info(),
//         token_metadata_program: ctx.accounts.token_metadata_program.to_account_info(),
//         token_program: ctx.accounts.token_program.to_account_info(),
//         system_program: ctx.accounts.system_program.to_account_info(),
//         recent_slothashes: ctx.accounts.recent_slothashes.to_account_info(),
//     };

//     let cpi_ctx = CpiContext::new_with_signer(candy_machine_program, mint_ix, &signer);

//     mpl_candy_machine_core::cpi::mint(cpi_ctx)
// }

#[derive(Debug, Clone)]
pub struct Token;

impl anchor_lang::Id for Token {
    fn id() -> Pubkey {
        spl_token::id()
    }
}

#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(
        seeds = [SEED, candy_guard.base.key().as_ref()],
        bump = candy_guard.bump
    )]
    pub candy_guard: Account<'info, CandyGuard>,
    /// CHECK: account constraints checked in account trait
    #[account(address = assetmanager::id())]
    pub candy_machine_program: AccountInfo<'info>,
    #[account(
        mut,
        constraint = candy_guard.key() == candy_machine.candy_guard.unwrap()
    )]
    pub candy_machine: Box<Account<'info, Asset>>,
    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    /// CHECK: account constraints checked in account trait
    #[account(address = sysvar::instructions::id())]
    pub instruction_sysvar_account: UncheckedAccount<'info>,
}
