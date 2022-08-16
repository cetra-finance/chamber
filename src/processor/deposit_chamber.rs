use crate::{
    handler::{deposit_borrow_tulip_levfarm, DepositBorrowTulipLevfarmAccounts},
    ChamberError, DepositChamber,
};
use anchor_lang::prelude::*;
use tulipv2_sdk_common::{
    math::common::{TryAdd, TryDiv, TryMul},
    pyth,
};

impl<'c, 'info> DepositChamber<'info> {
    pub fn process(
        &mut self,
        remaining_accounts: &'c [AccountInfo<'info>],
        base_amount: u64,
        quote_amount: u64,
    ) -> Result<()> {
        // TODO: Improve base checks
        // TODO: Enhance state management
        // TODO: Improve calculation & optimization

        if base_amount > self.user_position.base_amount
            || quote_amount > self.user_position.quote_amount
        {
            return Err(ChamberError::InsufficientUserPositionFunds.into());
        }

        let actual_base_amount = if base_amount > 0 {
            base_amount
                .checked_div(4)
                .ok_or(ChamberError::MathOverflow)?
        } else {
            0
        };

        let actual_quote_amount = if quote_amount > 0 {
            quote_amount
                .checked_div(4)
                .ok_or(ChamberError::MathOverflow)?
        } else {
            0
        };

        let base_price_account = &remaining_accounts[8];
        let quote_price_account = &remaining_accounts[9];

        let base_price = pyth::load_pyth_price(base_price_account.data.borrow().as_ref())?;
        // TODO: Add mint decimals lookup
        let base_decimals = 10u64.pow(9u32);

        let quote_price = pyth::load_pyth_price(quote_price_account.data.borrow().as_ref())?;
        // TODO: Add mint decimals lookup
        let quote_decimals = 10u64.pow(6u32);

        let total_base_value = base_price
            .try_mul(actual_base_amount)?
            .try_div(base_decimals)?;
        let total_quote_value = quote_price
            .try_mul(actual_quote_amount)?
            .try_div(quote_decimals)?;
        let total_deposit_value = total_base_value.try_add(total_quote_value)?;
        let max_deposit_value = total_deposit_value.try_mul(2)?;

        // TODO: Change calculation to actual quote token amount
        let quote_borrow_amount = max_deposit_value
            .try_div(quote_price)?
            .try_mul(quote_decimals)?
            .try_floor_u64()?;

        // Enter 1st position
        deposit_borrow_tulip_levfarm(
            Box::new(DepositBorrowTulipLevfarmAccounts {
                chamber: &self.chamber,
                chamber_authority: &self.authority,
                chamber_farm: &remaining_accounts[0],
                leveraged_farm: &remaining_accounts[1],
                chamber_farm_obligation: &remaining_accounts[2],
                coin_source_token_account: &self.chamber_base_ata.to_account_info(),
                coin_destination_token_account: &remaining_accounts[4],
                pc_source_token_account: &self.chamber_quote_ata.to_account_info(),
                pc_destination_token_account: &remaining_accounts[5],
                coin_deposit_reserve_account: &remaining_accounts[6],
                pc_deposit_reserve_account: &remaining_accounts[7],
                coin_reserve_liquidity_oracle: &remaining_accounts[8],
                pc_reserve_liquidity_oracle: &remaining_accounts[9],
                lending_market_account: &remaining_accounts[10],
                derived_lending_market_authority: &remaining_accounts[11],
                token_program: &self.token_program,
                lending_program: &remaining_accounts[12],
                coin_source_reserve_liquidity_token_account: &remaining_accounts[13],
                pc_source_reserve_liquidity_token_account: &remaining_accounts[14],
                coin_reserve_liquidity_fee_receiver: &remaining_accounts[15],
                pc_reserve_liquidity_fee_receiver: &remaining_accounts[16],
                borrow_authorizer: &remaining_accounts[17],
                lp_pyth_price_account: &remaining_accounts[18],
                vault_account: &remaining_accounts[19],
                position_info_account: &remaining_accounts[20],
                rent_sysvar: &self.rent_sysvar,
                levfarm_program: &remaining_accounts[22],
                system_program: &self.system_program,
            }),
            actual_base_amount,  //  SOL
            actual_quote_amount, //  USDC
            0,
            quote_borrow_amount,
            0,
        )?;

        let actual_base_amount = base_amount - actual_base_amount;
        let actual_quote_amount = quote_amount - actual_quote_amount;

        let total_base_value = base_price
            .try_mul(actual_base_amount)?
            .try_div(base_decimals)?;
        let total_quote_value = quote_price
            .try_mul(actual_quote_amount)?
            .try_div(quote_decimals)?;
        let total_deposit_value = total_base_value.try_add(total_quote_value)?;
        let max_deposit_value = total_deposit_value.try_mul(2)?;

        let base_borrow_amount = max_deposit_value
            .try_div(base_price)?
            .try_mul(base_decimals)?
            .try_floor_u64()?;

        // Enter 2nd position
        deposit_borrow_tulip_levfarm(
            Box::new(DepositBorrowTulipLevfarmAccounts {
                chamber: &self.chamber,
                chamber_authority: &self.authority,
                chamber_farm: &remaining_accounts[0],
                leveraged_farm: &remaining_accounts[1],
                chamber_farm_obligation: &remaining_accounts[3],
                coin_source_token_account: &self.chamber_base_ata.to_account_info(),
                coin_destination_token_account: &remaining_accounts[4],
                pc_source_token_account: &self.chamber_quote_ata.to_account_info(),
                pc_destination_token_account: &remaining_accounts[5],
                coin_deposit_reserve_account: &remaining_accounts[6],
                pc_deposit_reserve_account: &remaining_accounts[7],
                coin_reserve_liquidity_oracle: &remaining_accounts[8],
                pc_reserve_liquidity_oracle: &remaining_accounts[9],
                lending_market_account: &remaining_accounts[10],
                derived_lending_market_authority: &remaining_accounts[11],
                token_program: &self.token_program,
                lending_program: &remaining_accounts[12],
                coin_source_reserve_liquidity_token_account: &remaining_accounts[13],
                pc_source_reserve_liquidity_token_account: &remaining_accounts[14],
                coin_reserve_liquidity_fee_receiver: &remaining_accounts[15],
                pc_reserve_liquidity_fee_receiver: &remaining_accounts[16],
                borrow_authorizer: &remaining_accounts[17],
                lp_pyth_price_account: &remaining_accounts[18],
                vault_account: &remaining_accounts[19],
                position_info_account: &remaining_accounts[21],
                rent_sysvar: &self.rent_sysvar,
                levfarm_program: &remaining_accounts[22],
                system_program: &self.system_program,
            }),
            actual_base_amount,  //  SOL
            actual_quote_amount, //  USDC
            base_borrow_amount,
            0,
            1,
        )?;

        Ok(())
    }
}
