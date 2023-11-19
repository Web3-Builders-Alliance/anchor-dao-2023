use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::{config::DaoConfig, StakeState};

#[derive(Accounts)]
pub struct InitializeStake<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    owner_ata: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = owner,
        seeds = [b"vault", config.key().as_ref(), owner.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = stake_auth
    )]
    stake_ata: Account<'info, TokenAccount>,
    #[account(
        seeds=[b"auth", config.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    ///CHECK: This is safe. It's just used to sign things
    stake_auth: UncheckedAccount<'info>,
    #[account(
        seeds=[b"mint", config.key().as_ref()],
        bump = config.mint_bump
    )]
    mint: Account<'info, Mint>,
    #[account(
        init,
        payer = owner,
        seeds=[b"stake", config.key().as_ref(), owner.key().as_ref()],
        bump,
        space = StakeState::LEN
    )]
    stake_state: Account<'info, StakeState>,
    #[account(
        seeds=[b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    config: Account<'info, DaoConfig>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

impl<'info> InitializeStake<'info> {
    pub fn init(&mut self, bumps: &InitializeStakeBumps) -> Result<()> {
        self.stake_state.init(
            self.owner.key(),
            bumps.stake_state,
            bumps.stake_ata,
            bumps.stake_auth,
        )
    }
}
