use anchor_lang::prelude::*;

declare_id!("9GWa9HyP887TpVKAqk9qwd6TaCyVyULgARkEY4wCFDkm");


#[account]
pub struct Asset {
    pub authority: Pubkey,
    pub name: String,
    pub description: String,
    pub total_value: u64,
    pub total_fractions: u64,
    pub available_fractions: u64,
    pub price_per_fraction: u64,
    pub total_yield_distributed: u64,
    pub is_active: bool,
    pub bump: u8,
}

impl Asset {
    pub const LEN: usize = 8 + 32 + 64 + 256 + 8 + 8 + 8 + 8 + 8 + 1 + 1;
}

#[account]
pub struct InvestorPosition {
    pub investor: Pubkey,
    pub asset: Pubkey,
    pub fractions_owned: u64,
    pub yield_claimed: u64,
}

impl InvestorPosition {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8;
}



#[error_code]
pub enum RwaError {
    #[msg("Not enough fractions available")]
    NotEnoughFractions,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Asset is not active")]
    AssetNotActive,
    #[msg("No fractions owned")]
    NoFractionsOwned,
    #[msg("Name too long (max 64 chars)")]
    NameTooLong,
}



pub const ASSET_SEED: &[u8] = b"asset";
pub const POSITION_SEED: &[u8] = b"position";


#[program]
pub mod rwa_v2 {
    use super::*;


    pub fn create_asset(
        ctx: Context<CreateAsset>,
        name: String,
        description: String,
        total_value: u64,
        total_fractions: u64,
        price_per_fraction: u64,
    ) -> Result<()> {
        require!(name.len() <= 64, RwaError::NameTooLong);

        let asset = &mut ctx.accounts.asset;
        asset.authority = ctx.accounts.authority.key();
        asset.name = name;
        asset.description = description;
        asset.total_value = total_value;
        asset.total_fractions = total_fractions;
        asset.available_fractions = total_fractions;
        asset.price_per_fraction = price_per_fraction;
        asset.total_yield_distributed = 0;
        asset.is_active = true;
        asset.bump = ctx.bumps.asset;

        msg!("Asset created: {}", asset.name);
        Ok(())
    }


    pub fn buy_fractions(
        ctx: Context<BuyFractions>,
        amount: u64,
    ) -> Result<()> {
        let asset = &mut ctx.accounts.asset;

        require!(asset.is_active, RwaError::AssetNotActive);
        require!(asset.available_fractions >= amount, RwaError::NotEnoughFractions);

        let total_cost = asset.price_per_fraction
            .checked_mul(amount)
            .ok_or(RwaError::InsufficientFunds)?;

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.investor.key(),
            &ctx.accounts.authority.key(),
            total_cost,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.investor.to_account_info(),
                ctx.accounts.authority.to_account_info(),
            ],
        )?;

        asset.available_fractions -= amount;

        let position = &mut ctx.accounts.investor_position;
        position.investor = ctx.accounts.investor.key();
        position.asset = ctx.accounts.asset.key();
        position.fractions_owned += amount;

        msg!("Bought {} fractions", amount);
        Ok(())
    }

    pub fn distribute_yield(
        ctx: Context<DistributeYield>,
        total_yield: u64,
    ) -> Result<()> {
        let position = &ctx.accounts.investor_position;
        let asset = &mut ctx.accounts.asset;

        require!(position.fractions_owned > 0, RwaError::NoFractionsOwned);

        let investor_share = total_yield
            .checked_mul(position.fractions_owned)
            .unwrap()
            .checked_div(asset.total_fractions)
            .unwrap();

        asset.total_yield_distributed += investor_share;

        msg!("Yield share: {} lamports", investor_share);
        Ok(())
    }
}


#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateAsset<'info> {
    #[account(
        init,
        payer = authority,
        space = Asset::LEN,
        seeds = [ASSET_SEED, authority.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub asset: Account<'info, Asset>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyFractions<'info> {
    #[account(mut)]
    pub asset: Account<'info, Asset>,

    #[account(
        init_if_needed,
        payer = investor,
        space = InvestorPosition::LEN,
        seeds = [POSITION_SEED, investor.key().as_ref(), asset.key().as_ref()],
        bump
    )]
    pub investor_position: Account<'info, InvestorPosition>,

    #[account(mut)]
    pub investor: Signer<'info>,

    /// CHECK: получает оплату
    #[account(mut, address = asset.authority)]
    pub authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DistributeYield<'info> {
    #[account(mut, has_one = authority)]
    pub asset: Account<'info, Asset>,

    pub investor_position: Account<'info, InvestorPosition>,

    pub authority: Signer<'info>,
}