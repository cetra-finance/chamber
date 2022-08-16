use crate::{
    handler::{
        add_liquidity_raydium_tulip_levfarm, swap_tokens_raydium_tulip_levfarm,
        AddLiquidityRaydiumTulipLevfarmAccounts, SwapTokensRaydiumTulipLevfarmAccounts,
    },
    state, ChamberError, SettleChamberPosition,
};
use anchor_lang::prelude::*;

impl<'c, 'info> SettleChamberPosition<'info> {
    pub fn process(&mut self, remaining_accounts: &'c [AccountInfo<'info>]) -> Result<()> {
        // TODO: Extend protocols support
        if self.chamber.protocol_type != state::ProtocolType::Tulip {
            return Err(ChamberError::UnsupportedProtocol.into());
        }

        // TODO: Support more AMM's

        // 1. Swap tokens via Raydium
        swap_tokens_raydium_tulip_levfarm(
            Box::new(SwapTokensRaydiumTulipLevfarmAccounts {
                chamber: &self.chamber,
                chamber_authority: &self.authority,
                leveraged_farm: &remaining_accounts[0],
                chamber_farm: &remaining_accounts[1],
                chamber_farm_obligation: &remaining_accounts[2],
                token_program: &self.token_program,
                vault_signer: &remaining_accounts[17],
                swap_or_liquidity_program_id: &remaining_accounts[30],
                amm_id: &remaining_accounts[4],
                amm_authority: &remaining_accounts[5],
                amm_open_orders: &remaining_accounts[6],
                amm_quantities_or_target_orders: &remaining_accounts[7],
                pool_coin_token_account: &remaining_accounts[8],
                pool_pc_token_account: &remaining_accounts[9],
                serum_program_id: &remaining_accounts[10],
                serum_market: &remaining_accounts[11],
                serum_bids: &remaining_accounts[12],
                serum_asks: &remaining_accounts[13],
                serum_event_queue: &remaining_accounts[14],
                serum_coin_vault_account: &remaining_accounts[15],
                serum_pc_vault_account: &remaining_accounts[16],
                coin_wallet: &remaining_accounts[18],
                pc_wallet: &remaining_accounts[19],
                lending_market: &remaining_accounts[20],
                lending_market_authority: &remaining_accounts[21],
                lending_program: &remaining_accounts[22],
                position_info: &remaining_accounts[23],
                levfarm_program: &remaining_accounts[29],
            }),
            0,
        )?;

        swap_tokens_raydium_tulip_levfarm(
            Box::new(SwapTokensRaydiumTulipLevfarmAccounts {
                chamber: &self.chamber,
                chamber_authority: &self.authority,
                leveraged_farm: &remaining_accounts[0],
                chamber_farm: &remaining_accounts[1],
                chamber_farm_obligation: &remaining_accounts[3],
                token_program: &self.token_program,
                vault_signer: &remaining_accounts[17],
                swap_or_liquidity_program_id: &remaining_accounts[30],
                amm_id: &remaining_accounts[4],
                amm_authority: &remaining_accounts[5],
                amm_open_orders: &remaining_accounts[6],
                amm_quantities_or_target_orders: &remaining_accounts[7],
                pool_coin_token_account: &remaining_accounts[8],
                pool_pc_token_account: &remaining_accounts[9],
                serum_program_id: &remaining_accounts[10],
                serum_market: &remaining_accounts[11],
                serum_bids: &remaining_accounts[12],
                serum_asks: &remaining_accounts[13],
                serum_event_queue: &remaining_accounts[14],
                serum_coin_vault_account: &remaining_accounts[15],
                serum_pc_vault_account: &remaining_accounts[16],
                coin_wallet: &remaining_accounts[18],
                pc_wallet: &remaining_accounts[19],
                lending_market: &remaining_accounts[20],
                lending_market_authority: &remaining_accounts[21],
                lending_program: &remaining_accounts[22],
                position_info: &remaining_accounts[24],
                levfarm_program: &remaining_accounts[29],
            }),
            1,
        )?;

        // 2. Add tokens to liquidity pool
        add_liquidity_raydium_tulip_levfarm(
            Box::new(AddLiquidityRaydiumTulipLevfarmAccounts {
                chamber: &self.chamber,
                chamber_authority: &self.authority,
                chamber_farm: &remaining_accounts[1],
                leveraged_farm: &remaining_accounts[0],
                liquidity_program_id: &remaining_accounts[30],
                amm_id: &remaining_accounts[4],
                amm_authority: &remaining_accounts[5],
                amm_open_orders: &remaining_accounts[6],
                amm_quantities_or_target_orders: &remaining_accounts[7],
                lp_mint_address: &remaining_accounts[25],
                pool_coin_token_account: &remaining_accounts[8],
                pool_pc_token_account: &remaining_accounts[9],
                serum_market: &remaining_accounts[11],
                token_program: &self.token_program,
                lev_farm_coin_token_account: &remaining_accounts[18],
                lev_farm_pc_token_account: &remaining_accounts[19],
                lp_token_account: &remaining_accounts[26],
                pyth_price_account: &remaining_accounts[28],
                lending_market: &remaining_accounts[20],
                chamber_farm_obligation: &remaining_accounts[2],
                lending_market_authority: &remaining_accounts[21],
                lending_program: &remaining_accounts[22],
                clock_sysvar: &self.clock_sysvar,
                dex_program: &remaining_accounts[10],
                position_info: &remaining_accounts[23],
                levfarm_program: &remaining_accounts[29],
            }),
            0,
        )?;

        add_liquidity_raydium_tulip_levfarm(
            Box::new(AddLiquidityRaydiumTulipLevfarmAccounts {
                chamber: &self.chamber,
                chamber_authority: &self.authority,
                chamber_farm: &remaining_accounts[1],
                leveraged_farm: &remaining_accounts[0],
                liquidity_program_id: &remaining_accounts[30],
                amm_id: &remaining_accounts[4],
                amm_authority: &remaining_accounts[5],
                amm_open_orders: &remaining_accounts[6],
                amm_quantities_or_target_orders: &remaining_accounts[7],
                lp_mint_address: &remaining_accounts[25],
                pool_coin_token_account: &remaining_accounts[8],
                pool_pc_token_account: &remaining_accounts[9],
                serum_market: &remaining_accounts[11],
                token_program: &self.token_program,
                lev_farm_coin_token_account: &remaining_accounts[18],
                lev_farm_pc_token_account: &remaining_accounts[19],
                lp_token_account: &remaining_accounts[27],
                pyth_price_account: &remaining_accounts[28],
                lending_market: &remaining_accounts[20],
                chamber_farm_obligation: &remaining_accounts[3],
                lending_market_authority: &remaining_accounts[21],
                lending_program: &remaining_accounts[22],
                clock_sysvar: &self.clock_sysvar,
                dex_program: &remaining_accounts[10],
                position_info: &remaining_accounts[24],
                levfarm_program: &remaining_accounts[29],
            }),
            1,
        )?;

        Ok(())
    }
}
