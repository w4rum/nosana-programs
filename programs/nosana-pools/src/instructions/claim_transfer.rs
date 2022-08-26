use crate::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};
use nosana_rewards::cpi::accounts::AddFee;
use nosana_rewards::program::NosanaRewards;
use nosana_rewards::StatsAccount;

#[derive(Accounts)]
pub struct ClaimTransfer<'info> {
    #[account(mut, address = pool.vault @ NosanaError::InvalidTokenAccount)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = Clock::get()?.unix_timestamp > pool.start_time @ NosanaError::PoolNotStarted,
        constraint = pool.claim_type == ClaimType::Transfer @ NosanaError::PoolWrongClaimType,
    )]
    pub pool: Account<'info, PoolAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ClaimTransfer >) -> Result<()> {
    let now: i64 = Clock::get()?.unix_timestamp;
    let pool: &mut Account<PoolAccount> = &mut ctx.accounts.pool;
    let vault: &mut Account<TokenAccount> = &mut ctx.accounts.vault;

    let amount = pool.claim(vault.amount, now);

    // TODO: the below is not a requirement anymore, can be removed?
    // the pool must have enough funds for an emmission
    require!(amount >= pool.emmission, NosanaError::PoolUnderfunded);

    // transfer tokens from the vault back to the user
    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: vault.to_account_info(),
                to: ctx.accounts.user.to_account_info(),
                authority: vault.to_account_info(),
            },
            &[&[b"vault".as_ref(), pool.key().as_ref(), &[pool.vault_bump]]],
        ),
        amount,
    )
}
