use crate::{state, utils};
use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::token::Token;
use tulipv2_sdk_levfarm::instructions::deposit_borrow_dual::{
    deposit_borrow_dual, DepositBorrowDual,
};

pub struct DepositBorrowTulipLevfarmAccounts<'c, 'info> {
    pub chamber: &'c Account<'info, state::Chamber>,
    pub chamber_authority: &'c AccountInfo<'info>,
    pub chamber_farm: &'c AccountInfo<'info>,
    pub leveraged_farm: &'c AccountInfo<'info>,
    pub chamber_farm_obligation: &'c AccountInfo<'info>,
    pub coin_source_token_account: &'c AccountInfo<'info>,
    pub coin_destination_token_account: &'c AccountInfo<'info>,
    pub pc_source_token_account: &'c AccountInfo<'info>,
    pub pc_destination_token_account: &'c AccountInfo<'info>,
    pub coin_deposit_reserve_account: &'c AccountInfo<'info>,
    pub pc_deposit_reserve_account: &'c AccountInfo<'info>,
    pub coin_reserve_liquidity_oracle: &'c AccountInfo<'info>,
    pub pc_reserve_liquidity_oracle: &'c AccountInfo<'info>,
    pub lending_market_account: &'c AccountInfo<'info>,
    pub derived_lending_market_authority: &'c AccountInfo<'info>,
    pub token_program: &'c Program<'info, Token>,
    pub lending_program: &'c AccountInfo<'info>,
    pub coin_source_reserve_liquidity_token_account: &'c AccountInfo<'info>,
    pub pc_source_reserve_liquidity_token_account: &'c AccountInfo<'info>,
    pub coin_reserve_liquidity_fee_receiver: &'c AccountInfo<'info>,
    pub pc_reserve_liquidity_fee_receiver: &'c AccountInfo<'info>,
    pub borrow_authorizer: &'c AccountInfo<'info>,
    pub lp_pyth_price_account: &'c AccountInfo<'info>,
    pub vault_account: &'c AccountInfo<'info>,
    pub position_info_account: &'c AccountInfo<'info>,
    pub rent_sysvar: &'c Sysvar<'info, Rent>,
    pub levfarm_program: &'c AccountInfo<'info>,
    pub system_program: &'c Program<'info, System>,
}

pub fn deposit_borrow_tulip_levfarm<'c, 'info>(
    accounts: Box<DepositBorrowTulipLevfarmAccounts<'c, 'info>>,
    coin_amount: u64,
    pc_amount: u64,
    coin_borrow_amount: u64,
    pc_borrow_amount: u64,
    obligation_index: u8,
) -> Result<()> {
    let ix = deposit_borrow_dual(
        DepositBorrowDual {
            authority: accounts.chamber_authority.key(),
            user_farm: accounts.chamber_farm.key(),
            leveraged_farm: accounts.leveraged_farm.key(),
            user_farm_obligation: accounts.chamber_farm_obligation.key(),
            coin_source_token_account: accounts.coin_source_token_account.key(),
            coin_destination_token_account: accounts.coin_destination_token_account.key(),
            pc_source_token_account: accounts.pc_source_token_account.key(),
            pc_destination_token_account: accounts.pc_destination_token_account.key(),
            coin_deposit_reserve_account: accounts.coin_deposit_reserve_account.key(),
            pc_deposit_reserve_account: accounts.pc_deposit_reserve_account.key(),
            coin_reserve_liquidity_oracle: accounts.coin_reserve_liquidity_oracle.key(),
            pc_reserve_liquidity_oracle: accounts.pc_reserve_liquidity_oracle.key(),
            lending_market_account: accounts.lending_market_account.key(),
            derived_lending_market_authority: accounts.derived_lending_market_authority.key(),
            token_program: accounts.token_program.key(),
            lending_program: accounts.lending_program.key(),
            coin_source_reserve_liquidity_token_account: accounts
                .coin_source_reserve_liquidity_token_account
                .key(),
            pc_source_reserve_liquidity_token_account: accounts
                .pc_source_reserve_liquidity_token_account
                .key(),
            coin_reserve_liquidity_fee_receiver: accounts.coin_reserve_liquidity_fee_receiver.key(),
            pc_reserve_liquidity_fee_receiver: accounts.pc_reserve_liquidity_fee_receiver.key(),
            borrow_authorizer: accounts.borrow_authorizer.key(),
            lp_pyth_price_account: accounts.lp_pyth_price_account.key(),
            vault_account: accounts.vault_account.key(),
            rent: accounts.rent_sysvar.key(),
        },
        accounts.position_info_account.key(),
        accounts.system_program.key(),
        coin_amount,
        pc_amount,
        coin_borrow_amount,
        pc_borrow_amount,
        obligation_index,
    )
    .unwrap();

    invoke_signed(
        &ix,
        &Box::new(vec![
            accounts.chamber_authority.to_account_info(),
            accounts.chamber_farm.to_account_info(),
            accounts.leveraged_farm.to_account_info(),
            accounts.chamber_farm_obligation.to_account_info(),
            accounts.coin_source_token_account.to_account_info(),
            accounts.coin_destination_token_account.to_account_info(),
            accounts.pc_source_token_account.to_account_info(),
            accounts.pc_destination_token_account.to_account_info(),
            accounts.coin_deposit_reserve_account.to_account_info(),
            accounts.pc_deposit_reserve_account.to_account_info(),
            accounts.coin_reserve_liquidity_oracle.to_account_info(),
            accounts.pc_reserve_liquidity_oracle.to_account_info(),
            accounts.lending_market_account.to_account_info(),
            accounts.derived_lending_market_authority.to_account_info(),
            accounts.token_program.to_account_info(),
            accounts.lending_program.to_account_info(),
            accounts
                .coin_source_reserve_liquidity_token_account
                .to_account_info(),
            accounts
                .pc_source_reserve_liquidity_token_account
                .to_account_info(),
            accounts
                .coin_reserve_liquidity_fee_receiver
                .to_account_info(),
            accounts.pc_reserve_liquidity_fee_receiver.to_account_info(),
            accounts.borrow_authorizer.to_account_info(),
            accounts.lp_pyth_price_account.to_account_info(),
            accounts.vault_account.to_account_info(),
            accounts.rent_sysvar.to_account_info(),
            accounts.position_info_account.to_account_info(),
            accounts.system_program.to_account_info(),
        ]),
        &[&[
            utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
            accounts.chamber.key().as_ref(),
            &[accounts.chamber.authority_bump],
        ]],
    )?;

    Ok(())
}
