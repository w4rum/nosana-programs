use crate::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(
        mut,
        close = authority,
        has_one = authority @ NosanaError::Unauthorized,
        has_one = vault @ NosanaError::InvalidVault,
    )]
    pub market: Account<'info, MarketAccount>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Close<'info> {
    pub fn handler(&self) -> Result<()> {
        utils::cpi_transfer_tokens(
            self.vault.to_account_info(),
            self.user.to_account_info(),
            self.vault.to_account_info(),
            self.token_program.to_account_info(),
            seeds!(self.market),
            self.vault.amount,
        )?;
        utils::cpi_close_token_account(
            self.vault.to_account_info(),
            self.authority.to_account_info(),
            self.vault.to_account_info(),
            self.token_program.to_account_info(),
            seeds!(self.market),
        )
    }
}
