use crate::action::*;
use crate::errors::OCPErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::program_option::COption;
use anchor_lang::solana_program::sysvar;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use community_managed_token::instruction::create_transfer_with_delegate_instruction;

#[derive(Accounts)]
pub struct TransferCtx<'info> {
    policy: Box<Account<'info, Policy>>,
    /// CHECK: Checked in cpi
    freeze_authority: UncheckedAccount<'info>,
    #[account(
        constraint = mint_state.mint == mint.key() @ OCPErrorCode::InvalidMint,
        constraint = mint.key() == from_account.mint @ OCPErrorCode::InvalidMint,
        constraint = mint_state.locked_by.is_none() @ OCPErrorCode::MintStateLocked,
        constraint = mint.freeze_authority == COption::Some(freeze_authority.key()) @ OCPErrorCode::InvalidPolicyMintAssociation,
        constraint = mint.mint_authority == COption::Some(freeze_authority.key()) @ OCPErrorCode::InvalidPolicyMintAssociation,
        constraint = policy.get_freeze_authority(policy.key()) == freeze_authority.key() @ OCPErrorCode::InvalidPolicyMintAssociation,
    )]
    mint: Box<Account<'info, Mint>>,
    /// CHECK: going to check in action ctx
    metadata: UncheckedAccount<'info>,
    #[account(mut)]
    mint_state: Box<Account<'info, MintState>>,
    from: Signer<'info>, // either the owner of the token or the delegate of the token
    #[account(mut)]
    from_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: checked in cpi
    to: UncheckedAccount<'info>,
    #[account(mut, constraint = to_account.owner == to.key() @ OCPErrorCode::InvalidTokenAccount)]
    to_account: Box<Account<'info, TokenAccount>>,
    token_program: Program<'info, Token>,
    /// CHECK: checked in cpi
    #[account(address = community_managed_token::id())]
    cmt_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because the ID is checked with instructions sysvar
    #[account(address = sysvar::instructions::id())]
    instructions: UncheckedAccount<'info>,
}

impl From<&mut TransferCtx<'_>> for ActionCtx {
    fn from(ctx: &mut TransferCtx) -> Self {
        let mut action_ctx = ActionCtx {
            action: "transfer".to_string(),
            program_ids: vec![],
            last_memo_data: None,
            last_memo_signer: None,
            payer: None,
            from: Some(ctx.from.key().to_string()),
            to: Some(ctx.to.key().to_string()),
            mint: ctx.mint.key().to_string(),
            metadata: Some(to_metadata_ctx(&ctx.mint.key(), &ctx.metadata).expect("invalid metadata")),
            mint_account: Some(ctx.mint.clone().into()),
            mint_state: ctx.mint_state.clone().into_inner().into(),
        };
        action_ctx
            .parse_instructions(&ctx.instructions)
            .expect("failed to parse sysvar instructions");
        action_ctx
    }
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, TransferCtx<'info>>) -> Result<()> {
    let action_ctx: ActionCtx = ctx.accounts.into();
    ctx.accounts.policy.matches(&action_ctx)?;

    invoke_signed(
        &create_transfer_with_delegate_instruction(
            &ctx.accounts.from_account.owner,
            &ctx.accounts.to_account.owner,
            &ctx.accounts.from.key(), // from can be either the delegate or the owner of the from_account
            &ctx.accounts.mint.key(),
            &ctx.accounts.policy.key(),
            1,
        )?,
        &[
            ctx.accounts.from_account.to_account_info(),
            ctx.accounts.to_account.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.from.to_account_info(),
            ctx.accounts.policy.to_account_info(),
            ctx.accounts.freeze_authority.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.cmt_program.to_account_info(),
        ],
        &[&ctx.accounts.policy.signer_seeds()],
    )?;

    ctx.accounts.mint_state.record_transfer();

    Ok(())
}
