use crate::{state, utils};
use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::token::Token;
use tulipv2_sdk_levfarm::instructions::swap_tokens_raydium_stats::{
    swap_tokens_raydium_stats, RaydiumSwap,
};

pub struct SwapTokensRaydiumTulipLevfarmAccounts<'c, 'info> {
    pub chamber: &'c Account<'info, state::Chamber>,
    pub chamber_authority: &'c AccountInfo<'info>,
    pub leveraged_farm: &'c AccountInfo<'info>,
    pub chamber_farm: &'c AccountInfo<'info>,
    pub chamber_farm_obligation: &'c AccountInfo<'info>,
    pub token_program: &'c Program<'info, Token>,
    pub vault_signer: &'c AccountInfo<'info>,
    pub swap_or_liquidity_program_id: &'c AccountInfo<'info>,
    pub amm_id: &'c AccountInfo<'info>,
    pub amm_authority: &'c AccountInfo<'info>,
    pub amm_open_orders: &'c AccountInfo<'info>,
    pub amm_quantities_or_target_orders: &'c AccountInfo<'info>,
    pub pool_coin_token_account: &'c AccountInfo<'info>,
    pub pool_pc_token_account: &'c AccountInfo<'info>,
    pub serum_program_id: &'c AccountInfo<'info>,
    pub serum_market: &'c AccountInfo<'info>,
    pub serum_bids: &'c AccountInfo<'info>,
    pub serum_asks: &'c AccountInfo<'info>,
    pub serum_event_queue: &'c AccountInfo<'info>,
    pub serum_coin_vault_account: &'c AccountInfo<'info>,
    pub serum_pc_vault_account: &'c AccountInfo<'info>,
    pub coin_wallet: &'c AccountInfo<'info>,
    pub pc_wallet: &'c AccountInfo<'info>,
    pub lending_market: &'c AccountInfo<'info>,
    pub lending_market_authority: &'c AccountInfo<'info>,
    pub lending_program: &'c AccountInfo<'info>,
    pub position_info: &'c AccountInfo<'info>,
    pub levfarm_program: &'c AccountInfo<'info>,
}

#[inline(always)]
pub fn swap_tokens_raydium_tulip_levfarm<'c, 'info>(
    accounts: Box<SwapTokensRaydiumTulipLevfarmAccounts>,
    obligation_index: u8,
) -> Result<()> {
    invoke_signed(
        &swap_tokens_raydium_stats(
            Box::new(RaydiumSwap {
                authority: accounts.chamber_authority.key(),
                leveraged_farm: accounts.leveraged_farm.key(),
                user_farm: accounts.chamber_farm.key(),
                user_farm_obligation: accounts.chamber_farm_obligation.key(),
                token_program: accounts.token_program.key(),
                vault_signer: accounts.vault_signer.key(),
                swap_or_liquidity_program_id: accounts.swap_or_liquidity_program_id.key(),
                amm_id: accounts.amm_id.key(),
                amm_authority: accounts.amm_authority.key(),
                amm_open_orders: accounts.amm_open_orders.key(),
                amm_quantities_or_target_orders: accounts.amm_quantities_or_target_orders.key(),
                pool_coin_tokenaccount: accounts.pool_coin_token_account.key(),
                pool_pc_tokenaccount: accounts.pool_pc_token_account.key(),
                serum_program_id: accounts.serum_program_id.key(),
                serum_market: accounts.serum_market.key(),
                serum_bids: accounts.serum_bids.key(),
                serum_asks: accounts.serum_asks.key(),
                serum_event_queue: accounts.serum_event_queue.key(),
                serum_coin_vault_account: accounts.serum_coin_vault_account.key(),
                serum_pc_vault_account: accounts.serum_pc_vault_account.key(),
                serum_vault_signer: accounts.vault_signer.key(),
                coin_wallet: accounts.coin_wallet.key(),
                pc_wallet: accounts.pc_wallet.key(),
            }),
            accounts.lending_market.key(),
            accounts.lending_market_authority.key(),
            accounts.lending_program.key(),
            accounts.position_info.key(),
            obligation_index,
        )
        .unwrap(),
        &Box::new(vec![
            accounts.chamber_authority.to_account_info(),
            accounts.leveraged_farm.to_account_info(),
            accounts.chamber_farm.to_account_info(),
            accounts.chamber_farm_obligation.to_account_info(),
            accounts.token_program.to_account_info(),
            accounts.leveraged_farm.to_account_info(),
            accounts.swap_or_liquidity_program_id.to_account_info(),
            accounts.amm_id.to_account_info(),
            accounts.amm_authority.to_account_info(),
            accounts.amm_open_orders.to_account_info(),
            accounts.amm_quantities_or_target_orders.to_account_info(),
            accounts.pool_coin_token_account.to_account_info(),
            accounts.pool_pc_token_account.to_account_info(),
            accounts.serum_program_id.to_account_info(),
            accounts.serum_market.to_account_info(),
            accounts.serum_bids.to_account_info(),
            accounts.serum_asks.to_account_info(),
            accounts.serum_event_queue.to_account_info(),
            accounts.serum_coin_vault_account.to_account_info(),
            accounts.serum_pc_vault_account.to_account_info(),
            accounts.vault_signer.to_account_info(),
            accounts.coin_wallet.to_account_info(),
            accounts.pc_wallet.to_account_info(),
            accounts.lending_market.to_account_info(),
            accounts.lending_market_authority.to_account_info(),
            accounts.lending_program.to_account_info(),
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
