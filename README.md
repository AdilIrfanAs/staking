# Solana Anchor 

### Stake

```rust 
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

```
Here, our staking allows users to stake their SPL mint tokens, and these SPL mint tokens will be deposited into the program's Program-Derived Account (PDA). The program will also keep a record of staking activities for each user.

### Withdraw
```rust 
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

```
Allowing users to unstake their tokens and withdraw them along with the staking reward. The program will mint staking reward tokens according to a set formula.