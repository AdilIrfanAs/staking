use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

use std::mem::size_of;
declare_id!("51zhqoSMpTitAHigSEyxNkRhZRaouyWdgXZ8VUBXnkGH");

#[program]
pub mod staking {
    pub const MINT_ADDRESS: &str = "8pXMXHBBsbgaCZZPwneCBdkBfmnxiZ8HZZXnvsciwme6";

    use anchor_spl::token::accessor::amount;

    use super::*;
    pub fn intialize_user_account(ctx: Context<InitializeUserAccount>) -> Result<()> {
     //logic
        Ok(())
    }
    pub fn stake(
        ctx: Context<Stake>,
        program_bag_bump: u8,
        stake_count: String,
        amount: u64,
    ) -> Result<()> {
       //logic
        Ok(())
    }

    pub fn create_token_bag(ctx: Context<CreateFinoTokenBag>) -> Result<()> {
        //logic
        Ok(())
    }

}
#[account]
pub struct Pool {
    owner: Pubkey,
    amount: u64,
    start_time: i64,
    seed: u64,
}
#[account]
pub struct StakingAccount {
    total_staked_amount: u64,
    staked_accounts: u32,
}

#[derive(Accounts)]
#[instruction(program_bag_bump: u8,stake_count: u8)]
pub struct Stake<'info> {
    #[account(
        init,
        seeds = [b"stake".as_ref(), user.key().as_ref(),stake_count.as_ref()],
        bump,
        payer = user,
        space = size_of::<Pool>() + 16
    )]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        address = MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_account: Account<'info, StakingAccount>,
    #[account(mut)]
    pub user_token_bag: Account<'info, TokenAccount>,
    pub user_token_bag_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}
 

#[derive(Accounts)]
#[instruction(program_bag_bump: u8,mint_authority_bump:u8)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub user_account: Account<'info, StakingAccount>,
    pub token_program: Program<'info, Token>,

    #[account(
        mut,
        address = MINT_ADDRESS.parse::<Pubkey>().unwrap(),
        )]
    pub mint: Account<'info, Mint>,
    #[account(
        seeds = [mint.key().as_ref() ],
        bump = mint_authority_bump,
        )]
    pub mint_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub user_token_bag: Account<'info, TokenAccount>,
}

