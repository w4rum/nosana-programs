mod errors;
mod instructions;
mod macros;
mod security;
mod state;

use anchor_lang::prelude::*;
pub use errors::*; // expose errors for cpi
use instructions::*;
use nosana_common::*;
pub use state::*; // expose stake for cpi

declare_id!(id::STAKING_PROGRAM);

#[program]
pub mod nosana_staking {
    use super::*;

    /// Initialize the [SettingsAccount](#settings-account).
    pub fn init(ctx: Context<Init>) -> Result<()> {
        ctx.accounts.handler()
    }

    /// Create a [StakeAccount](#stake-account) and [VaultAccount](#vault-account).
    /// Stake `amount` of [NOS](/tokens/token) tokens for `duration` fo seconds.
    pub fn stake(ctx: Context<Stake>, amount: u64, duration: u128) -> Result<()> {
        ctx.accounts
            .handler(amount, duration, *ctx.bumps.get("vault").unwrap())
    }

    /// Start the unstake duration.
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.handler()
    }

    /// Make a stake active again and reset the unstake time.
    pub fn restake(ctx: Context<Restake>) -> Result<()> {
        ctx.accounts.handler()
    }

    /// Top-up `amount` of [NOS](/tokens/token) of a [StakeAccount](#stake-account).
    pub fn topup(ctx: Context<Topup>, amount: u64) -> Result<()> {
        ctx.accounts.handler(amount)
    }

    /// Extend the `duration` of a [StakeAccount](#stake-account).
    pub fn extend(ctx: Context<Extend>, duration: u64) -> Result<()> {
        ctx.accounts.handler(duration)
    }

    /// The `claim()` instruction will transfer back all your stake tokens if the delay has
    /// passed after they whey unstaked. Claiming will close the [StakeAccount](#stake-account)
    /// and [VaultAccount](#vault-account) of the staker.
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.handler()
    }

    /// Reduces a [StakeAccount](#stake-account)'s  [NOS](/tokens/token) tokens.
    /// This can only be done by the Slashing Authority.
    /// Slashing is a feature used by the Nosana Protocol to punish bad actors.
    pub fn slash(ctx: Context<Slash>, amount: u64) -> Result<()> {
        ctx.accounts.handler(amount)
    }

    /// Update the Slashing Authority and Token Account.
    pub fn update_settings(ctx: Context<UpdateSettings>) -> Result<()> {
        ctx.accounts.handler()
    }
}
