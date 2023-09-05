pub struct Swap<'info> {
    #[account(seeds=[INITIALIZE_POOL_TAG,pool.key().as_ref()],bump)]
    pub pool_authority: AccountInfo<'info>,
    pub pool: Account<'info, Pool>,
    /// Non-zero token A account
    #[account(owner=pool_authority.key())]
    pub pool_source_account: Account<'info, TokenAccount>,
    /// Non-zero token B account
    #[account(owner=pool_authority.key())]
    pub pool_destination_account: Account<'info, TokenAccount>,
    /// Non-zero token A account
    #[account(owner=pool_authority.key())]
    pub source_account: Account<'info, TokenAccount>,
    /// Non-zero token B account
    #[account(owner=pool_authority.key())]
    pub destination_account: Account<'info, TokenAccount>,
    #[account(owner=pool.key())]
    pub pool_mint: Account<'info, Mint>,
    // Token A mint
    pub source_mint: Account<'info, Mint>,
    // Token B mint
    pub destination_mint: Account<'info, Mint>,
    #[account(token::mint=pool_mint)]
    pub pool_token_fee_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
