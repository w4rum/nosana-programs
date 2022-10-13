use crate::*;
use anchor_spl::token::TokenAccount;
use mpl_token_metadata::pda::find_metadata_account;
use nosana_staking::StakeAccount;

#[derive(Accounts)]
pub struct Work<'info> {
    /// CHECK: the run account is created optionally
    #[account(
        mut,
        signer @ NosanaError::MissingSignature,
        owner = id::SYSTEM_PROGRAM @ NosanaError::InvalidOwner,
        constraint = run.lamports() == 0 @ NosanaError::LamportsNonNull
    )]
    pub run: AccountInfo<'info>,
    #[account(
        mut,
        constraint = MarketAccount::node_constraint(authority.key, &market.queue, market.queue_type)
            @ NosanaError::NodeAlreadyQueued
    )]
    pub market: Account<'info, MarketAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        address = utils::get_staking_address(authority.key) @ NosanaError::StakeDoesNotMatchReward,
        has_one = authority @ NosanaError::Unauthorized,
        constraint = stake.xnos >= market.node_xnos_minimum as u128 @ NosanaError::NodeNotEnoughStake,
    )]
    pub stake: Account<'info, StakeAccount>,
    #[account(constraint = nft.owner == authority.key() @ NosanaError::NodeNftWrongOwner)]
    pub nft: Account<'info, TokenAccount>,
    /// CHECK: Metaplex metadata is verified against NFT and Collection node access key
    #[account(
        address = find_metadata_account(&nft.mint).0 @ NosanaError::NodeNftWrongMetadata,
        constraint = MarketAccount::metadata_constraint(&metadata, market.node_access_key)
            @ NosanaError::NodeKeyInvalidCollection,
    )]
    pub metadata: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Work<'info> {
    fn handle_run_account(&mut self) -> Result<()> {
        // cpi system program
        RunAccount::cpi_init(
            self.run.to_account_info(),
            self.payer.to_account_info(),
            self.system_program.to_account_info(),
        );

        // get and modify run account
        let info: AccountInfo = self.run.to_account_info();
        let mut run: Account<RunAccount> = RunAccount::from_account_info(&info);
        run.create(
            self.market.pop_from_queue(),
            self.authority.key(),
            self.payer.key(),
            Clock::get().unwrap().unix_timestamp,
        )?;

        // serialize run account
        run.serialize(info)
    }

    pub fn handler(&mut self) -> Result<()> {
        match QueueType::from(self.market.queue_type) {
            QueueType::Node | QueueType::Empty => {
                self.market.add_to_queue(self.authority.key(), false)
            }
            QueueType::Job => self.handle_run_account(),
        }
    }
}
