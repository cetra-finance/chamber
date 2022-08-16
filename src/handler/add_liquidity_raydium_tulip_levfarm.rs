use crate::{state, utils};
use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::token::Token;
use tulipv2_sdk_levfarm::instructions::add_liquidity_stats::{add_liquidity_stats, AddLiquidity};

pub struct AddLiquidityRaydiumTulipLevfarmAccounts<'c, 'info> {
    pub chamber: &'c Account<'info, state::Chamber>,
    pub chamber_authority: &'c AccountInfo<'info>,
    pub chamber_farm: &'c AccountInfo<'info>,
    pub leveraged_farm: &'c AccountInfo<'info>,
    pub liquidity_program_id: &'c AccountInfo<'info>,
    pub amm_id: &'c AccountInfo<'info>,
    pub amm_authority: &'c AccountInfo<'info>,
    pub amm_open_orders: &'c AccountInfo<'info>,
    pub amm_quantities_or_target_orders: &'c AccountInfo<'info>,
    pub lp_mint_address: &'c AccountInfo<'info>,
    pub pool_coin_token_account: &'c AccountInfo<'info>,
    pub pool_pc_token_account: &'c AccountInfo<'info>,
    pub serum_market: &'c AccountInfo<'info>,
    pub token_program: &'c Program<'info, Token>,
    pub lev_farm_coin_token_account: &'c AccountInfo<'info>,
    pub lev_farm_pc_token_account: &'c AccountInfo<'info>,
    pub lp_token_account: &'c AccountInfo<'info>,
    pub pyth_price_account: &'c AccountInfo<'info>,
    pub lending_market: &'c AccountInfo<'info>,
    pub chamber_farm_obligation: &'c AccountInfo<'info>,
    pub lending_market_authority: &'c AccountInfo<'info>,
    pub lending_program: &'c AccountInfo<'info>,
    pub clock_sysvar: &'c Sysvar<'info, Clock>,
    pub dex_program: &'c AccountInfo<'info>,
    pub position_info: &'c AccountInfo<'info>,
    pub levfarm_program: &'c AccountInfo<'info>,
}

#[inline(always)]
pub fn add_liquidity_raydium_tulip_levfarm<'c, 'info>(
    accounts: Box<AddLiquidityRaydiumTulipLevfarmAccounts>,
    obligation_index: u8,
) -> Result<()> {
    invoke_signed(
        &add_liquidity_stats(
            Box::new(AddLiquidity {
                authority: accounts.chamber_authority.key(),
                user_farm: accounts.chamber_farm.key(),
                leveraged_farm: accounts.leveraged_farm.key(),
                liquidity_program_id: accounts.liquidity_program_id.key(),
                amm_id: accounts.amm_id.key(),
                amm_authority: accounts.amm_authority.key(),
                amm_open_orders: accounts.amm_open_orders.key(),
                amm_quantities_or_target_orders: accounts.amm_quantities_or_target_orders.key(),
                lp_mint_address: accounts.lp_mint_address.key(),
                pool_coin_token_account: accounts.pool_coin_token_account.key(),
                pool_pc_token_account: accounts.pool_pc_token_account.key(),
                serum_market: accounts.serum_market.key(),
                token_program: accounts.token_program.key(),
                lev_farm_coin_token_account: accounts.lev_farm_coin_token_account.key(),
                lev_farm_pc_token_account: accounts.lev_farm_pc_token_account.key(),
                user_lp_token_account: accounts.lp_token_account.key(),
                pyth_price_account: accounts.pyth_price_account.key(),
                lending_market_account: accounts.lending_market.key(),
                user_farm_obligation: accounts.chamber_farm_obligation.key(),
                derived_lending_market_authority: accounts.lending_market_authority.key(),
                lending_program: accounts.lending_program.key(),
                clock: accounts.clock_sysvar.key(),
                dex_program: accounts.dex_program.key(),
            }),
            accounts.position_info.key(),
            obligation_index,
        )
        .unwrap(),
        &Box::new(vec![
            accounts.chamber_authority.to_account_info(),
            accounts.chamber_farm.to_account_info(),
            accounts.leveraged_farm.to_account_info(),
            accounts.liquidity_program_id.to_account_info(),
            accounts.amm_id.to_account_info(),
            accounts.amm_authority.to_account_info(),
            accounts.amm_open_orders.to_account_info(),
            accounts.amm_quantities_or_target_orders.to_account_info(),
            accounts.lp_mint_address.to_account_info(),
            accounts.pool_coin_token_account.to_account_info(),
            accounts.pool_pc_token_account.to_account_info(),
            accounts.serum_market.to_account_info(),
            accounts.token_program.to_account_info(),
            accounts.lev_farm_coin_token_account.to_account_info(),
            accounts.lev_farm_pc_token_account.to_account_info(),
            accounts.lp_token_account.to_account_info(),
            accounts.pyth_price_account.to_account_info(),
            accounts.lending_market.to_account_info(),
            accounts.chamber_farm_obligation.to_account_info(),
            accounts.lending_market_authority.to_account_info(),
            accounts.lending_program.to_account_info(),
            accounts.clock_sysvar.to_account_info(),
            accounts.dex_program.to_account_info(),
            accounts.position_info.to_account_info(),
        ]),
        &[&[
            utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
            accounts.chamber.key().as_ref(),
            &[accounts.chamber.authority_bump],
        ]],
    )?;

    Ok(())
}
