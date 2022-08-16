pub mod error;
mod handler;
mod processor;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use error::ChamberError;

declare_id!("cmbrLdggVpadQMe54SMWVvSA6ajswMSBtwnLG2xyqZE");

solana_security_txt::security_txt! {
    name: "Cetra Chamber",
    project_url: "https://cetra.finance",
    contacts: "email:example@example.com,link:https://example.com/security,discord:example#1234",
    policy: "https://github.com/solana-labs/solana/blob/master/SECURITY.md"
}

#[program]
mod chamber {
    use super::*;

    pub fn initialize_chamber<'a, 'b, 'c, 'info>(
        ctx: Context<'_, '_, 'c, 'info, InitializeChamber<'info>>,
        leveraged_farm: Pubkey,
        bump: u8,
        authority_bump: u8,
        protocol_type: crate::state::ProtocolType,
    ) -> Result<()> {
        ctx.accounts
            .process(leveraged_farm, bump, authority_bump, protocol_type)
    }

    pub fn initialize_chamber_strategy<'a, 'b, 'c, 'info>(
        ctx: Context<'_, '_, 'c, 'info, InitializeChamberStrategy<'info>>,
    ) -> Result<()> {
        ctx.accounts.process(&ctx.remaining_accounts)
    }

    pub fn initialize_user_position<'a, 'b, 'c, 'info>(
        ctx: Context<'_, '_, 'c, 'info, InitializeUserPosition<'info>>,
        bump: u8,
        base_amount: u64,
        quote_amount: u64,
    ) -> Result<()> {
        ctx.accounts.process(bump, base_amount, quote_amount)
    }

    pub fn deposit_chamber<'a, 'b, 'c, 'info>(
        ctx: Context<'_, '_, 'c, 'info, DepositChamber<'info>>,
        base_amount: u64,
        quote_amount: u64,
    ) -> Result<()> {
        ctx.accounts
            .process(&ctx.remaining_accounts, base_amount, quote_amount)
    }

    pub fn settle_chamber_position<'a, 'b, 'c, 'info>(
        ctx: Context<'_, '_, 'c, 'info, SettleChamberPosition<'info>>,
    ) -> Result<()> {
        ctx.accounts.process(&ctx.remaining_accounts)
    }

    pub fn settle_chamber_position2<'a, 'b, 'c, 'info>(
        ctx: Context<'_, '_, 'c, 'info, SettleChamberPosition2<'info>>,
        nonce_0: u8,
        nonce_1: u8,
        meta_nonce_0: u8,
        meta_nonce_1: u8,
    ) -> Result<()> {
        ctx.accounts.process(
            &ctx.remaining_accounts,
            nonce_0,
            nonce_1,
            meta_nonce_0,
            meta_nonce_1,
        )
    }
}

#[derive(Accounts)]
#[instruction(leveraged_farm: Pubkey, bump: u8, authority_bump: u8, protocol_type: state::ProtocolType)]
pub struct InitializeChamber<'info> {
    /// Protocol agnostic `Vault` and strategy controller.
    #[account(init, seeds = [utils::CHAMBER_PREFIX.as_bytes(), leveraged_farm.key().as_ref()], bump, space = state::Chamber::LEN, payer = payer)]
    pub chamber: Box<Account<'info, state::Chamber>>,

    /// Chamber authority.
    #[account(seeds = [utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(), chamber.key().as_ref()], bump = authority_bump)]
    pub authority: UncheckedAccount<'info>,

    /// `Chamber` strategy uninitialized base associated token account.
    #[account(mut)]
    pub base_ata: UncheckedAccount<'info>,

    /// `Chamber` strategy uninitialized quote associated token account.
    #[account(mut)]
    pub quote_ata: UncheckedAccount<'info>,

    pub base_mint: Box<Account<'info, Mint>>,
    pub quote_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub rent_sysvar: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeChamberStrategy<'info> {
    /// Protocol agnostic `Vault` and strategy controller.
    #[account(seeds = [utils::CHAMBER_PREFIX.as_bytes(), chamber.leveraged_farm.key().as_ref()], bump = chamber.bump)]
    pub chamber: Box<Account<'info, state::Chamber>>,

    /// Chamber authority.
    #[account(mut, seeds = [utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(), chamber.key().as_ref()], bump = chamber.authority_bump)]
    pub authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub clock_sysvar: Sysvar<'info, Clock>,
    pub rent_sysvar: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    /*
    /// Accounts expected by Tulip:

    pub global: UncheckedAccount<'info>,

    /// Tulip leveraged `UserFarm` state.
    #[account(mut)]
    pub chamber_farm: UncheckedAccount<'info>,

    /// Tulip leveraged `Obligation` state for first position.
    #[account(mut)]
    pub chamber_farm_obligation_0: UncheckedAccount<'info>,

    /// Tulip leveraged `Obligation` state for second position.
    #[account(mut)]
    pub chamber_farm_obligation_1: UncheckedAccount<'info>,

    /// Tulip `Raydium` LP tokens state for first position, managed by `chamber_farm_obligation`.
    #[account(mut)]
    pub chamber_obligation_vault_0: UncheckedAccount<'info>,

    /// Tulip `Raydium` LP tokens state for second position, managed by `chamber_farm_obligation`.
    #[account(mut)]
    pub chamber_obligation_vault_1: UncheckedAccount<'info>,

    #[account(mut)]
    pub lending_market: UncheckedAccount<'info>,
    pub leveraged_farm: UncheckedAccount<'info>,

    /// Uninitialized associated token account which holds `Raydium` LP for first obligation position.
    #[account(mut)]
    pub raydium_lp_ata_0: UncheckedAccount<'info>,

    /// Uninitialized associated token account which holds `Raydium` LP for second obligation position.
    #[account(mut)]
    pub raydium_lp_ata_1: UncheckedAccount<'info>,

    /// `Raydium` LP mint.
    pub raydium_lp_mint: Box<Account<'info, Mint>>,

    /// Uninitialized associated token account which holds `Tulip` rewards for first obligation position.
    #[account(mut)]
    pub tulip_ata_0: UncheckedAccount<'info>,

    /// Uninitialized associated token account which holds `Tulip` rewards for second obligation position.
    #[account(mut)]
    pub tulip_ata_1: UncheckedAccount<'info>,

    /// `Tulip` token mint.
    pub tulip_mint: Box<Account<'info, Mint>>,

    pub lending_program: UncheckedAccount<'info>,
    pub levfarm_program: UncheckedAccount<'info>,
    pub solfarm_vault_program: UncheckedAccount<'info>
    */
}

#[derive(Accounts)]
#[instruction(bump: u8, base_amount: u64, quote_amount: u64)]
pub struct InitializeUserPosition<'info> {
    /// Represent `payer` position in provided `Chamber` strategy.
    #[account(init, seeds = [utils::USER_POSITION_PREFIX.as_bytes(), payer.key().as_ref(), chamber.key().as_ref()], bump, space = state::UserPosition::LEN, payer = payer)]
    pub user_position: Box<Account<'info, state::UserPosition>>,

    /// Protocol agnostic `Vault` and strategy controller.
    #[account(
        seeds = [utils::CHAMBER_PREFIX.as_bytes(), chamber.leveraged_farm.key().as_ref()],
        bump = chamber.bump,
        constraint = chamber.base_ata == chamber_base_ata.key(),
        constraint = chamber.quote_ata == chamber_quote_ata.key(),
    )]
    pub chamber: Box<Account<'info, state::Chamber>>,

    #[account(mut, constraint = user_base_ata.mint == chamber.base_mint)]
    pub user_base_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = user_quote_ata.mint == chamber.quote_mint)]
    pub user_quote_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub chamber_base_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub chamber_quote_ata: Box<Account<'info, TokenAccount>>,

    /// Alias for user.
    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(base_amount: u64, quote_amount: u64)]
pub struct DepositChamber<'info> {
    #[account(seeds = [utils::USER_POSITION_PREFIX.as_bytes(), payer.key().as_ref(), chamber.key().as_ref()], bump = user_position.bump)]
    pub user_position: Box<Account<'info, state::UserPosition>>,

    /// Protocol agnostic `Vault` and strategy controller.
    #[account(
        seeds = [utils::CHAMBER_PREFIX.as_bytes(), chamber.leveraged_farm.key().as_ref()],
        bump = chamber.bump,
        has_one = authority,
        constraint = chamber.base_ata == chamber_base_ata.key(),
        constraint = chamber.quote_ata == chamber_quote_ata.key(),
    )]
    pub chamber: Box<Account<'info, state::Chamber>>,

    /// Chamber authority.
    #[account(mut, seeds = [utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(), chamber.key().as_ref()], bump = chamber.authority_bump)]
    pub authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub chamber_base_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub chamber_quote_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub clock_sysvar: Sysvar<'info, Clock>,
    pub rent_sysvar: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    /*
    /// Accounts expected by Tulip:

    /// 0.
    #[account(mut)]
    pub chamber_farm: UncheckedAccount<'info>,

    /// 1.
    pub leveraged_farm: UncheckedAccount<'info>,

    /// 2.
    #[account(mut)]
    pub chamber_farm_obligation_0: UncheckedAccount<'info>,

    /// 3.
    #[account(mut)]
    pub chamber_farm_obligation_1: UncheckedAccount<'info>,

    /// 4.
    #[account(mut)]
    pub coin_destination_token_account: UncheckedAccount<'info>,

    /// 5.
    #[account(mut)]
    pub pc_destination_token_account: UncheckedAccount<'info>,

    /// 6.
    #[account(mut)]
    pub coin_deposit_reserve_account: UncheckedAccount<'info>,

    /// 7.
    #[account(mut)]
    pub pc_deposit_reserve_account: UncheckedAccount<'info>,

    /// 8.
    pub coin_reserve_liquidity_oracle: UncheckedAccount<'info>,

    /// 9.
    pub pc_reserve_liquidity_oracle: UncheckedAccount<'info>,

    /// 10.
    pub lending_market_account: UncheckedAccount<'info>,

    /// 11.
    pub derived_lending_market_authority: UncheckedAccount<'info>,

    /// 12.
    pub lending_program: UncheckedAccount<'info>,

    /// 13.
    #[account(mut)]
    pub coin_source_reserve_liquidity_token_account: UncheckedAccount<'info>,

    /// 14.
    #[account(mut)]
    pub pc_source_reserve_liquidity_token_account: UncheckedAccount<'info>,

    /// 15.
    #[account(mut)]
    pub coin_reserve_liquidity_fee_receiver: UncheckedAccount<'info>,

    /// 16.
    #[account(mut)]
    pub pc_reserve_liquidity_fee_receiver: UncheckedAccount<'info>,

    /// 17.
    pub borrow_authorizer: UncheckedAccount<'info>,

    /// 18.
    pub lp_pyth_price_account: UncheckedAccount<'info>,

    /// 19.
    #[account(mut)]
    pub vault_account: UncheckedAccount<'info>,

    /// 20.
    #[account(mut)]
    pub position_info_0: UncheckedAccount<'info>,

    /// 21.
    #[account(mut)]
    pub position_info_1: UncheckedAccount<'info>,

    /// 22.
    pub levfarm_program: UncheckedAccount<'info>,
    */
}

#[derive(Accounts)]
pub struct SettleChamberPosition<'info> {
    /// Protocol agnostic `Vault` and strategy controller.
    #[account(
        seeds = [utils::CHAMBER_PREFIX.as_bytes(), chamber.leveraged_farm.key().as_ref()],
        bump = chamber.bump,
        has_one = authority,
    )]
    pub chamber: Box<Account<'info, state::Chamber>>,

    /// Chamber authority.
    #[account(mut, seeds = [utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(), chamber.key().as_ref()], bump = chamber.authority_bump)]
    pub authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub clock_sysvar: Sysvar<'info, Clock>,
    pub rent_sysvar: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    /*
    /// Accounts expected by Tulip:

    /// 0.
    #[account(mut)]
    pub leveraged_farm: UncheckedAccount<'info>,

    /// 1.
    #[account(mut)]
    pub chamber_farm: UncheckedAccount<'info>,

    /// 2.
    #[account(mut)]
    pub chamber_farm_obligation_0: UncheckedAccount<'info>,

    /// 3.
    #[account(mut)]
    pub chamber_farm_obligation_1: UncheckedAccount<'info>,

    /// 4.
    #[account(mut)]
    pub amm_id: UncheckedAccount<'info>,

    /// 5.
    #[account(mut)]
    pub amm_authority: UncheckedAccount<'info>,

    /// 6.
    #[account(mut)]
    pub amm_open_orders: UncheckedAccount<'info>,

    /// 7.
    #[account(mut)]
    pub amm_quantities_or_target_orders: UncheckedAccount<'info>,

    /// 8.
    #[account(mut)]
    pub pool_coin_token_account: UncheckedAccount<'info>,

    /// 9.
    #[account(mut)]
    pub pool_pc_token_account: UncheckedAccount<'info>,

    /// 10.
    pub serum_program_id: UncheckedAccount<'info>,

    /// 11.
    #[account(mut)]
    pub serum_market: UncheckedAccount<'info>,

    /// 12.
    #[account(mut)]
    pub serum_bids: UncheckedAccount<'info>,

    /// 13.
    #[account(mut)]
    pub serum_asks: UncheckedAccount<'info>,

    /// 14.
    #[account(mut)]
    pub serum_event_queue: UncheckedAccount<'info>,

    /// 15.
    #[account(mut)]
    pub serum_coin_vault_account: UncheckedAccount<'info>,

    /// 16.
    #[account(mut)]
    pub serum_pc_vault_account: UncheckedAccount<'info>,

    /// 17.
    #[account(mut)]
    pub vault_signer: UncheckedAccount<'info>,

    /// 18.
    #[account(mut)]
    pub coin_wallet: UncheckedAccount<'info>,

    /// 19.
    #[account(mut)]
    pub pc_wallet: UncheckedAccount<'info>,

    /// 20.
    pub lending_market: UncheckedAccount<'info>,

    /// 21.
    pub lending_market_authority: UncheckedAccount<'info>,

    /// 22.
    pub lending_program: UncheckedAccount<'info>,

    /// 23.
    #[account(mut)]
    pub position_info_0: UncheckedAccount<'info>,

    /// 24.
    #[account(mut)]
    pub position_info_1: UncheckedAccount<'info>,

    /// 25.
    #[account(mut)]
    pub lp_mint: UncheckedAccount<'info>,

    /// 26.
    #[account(mut)]
    pub lp_token_account_0: UncheckedAccount<'info>,

    /// 27.
    #[account(mut)]
    pub lp_token_account_1: UncheckedAccount<'info>,

    /// 28.
    pub pyth_price_account: UncheckedAccount<'info>,

    /// 29.
    pub levfarm_program: UncheckedAccount<'info>,

    /// 30.
    pub liquidity_program: UncheckedAccount<'info>,
    */
}

#[derive(Accounts)]
#[instruction(nonce_0: u8, nonce_1: u8, meta_nonce_0: u8, meta_nonce_1: u8)]
pub struct SettleChamberPosition2<'info> {
    /// Protocol agnostic `Vault` and strategy controller.
    #[account(
        seeds = [utils::CHAMBER_PREFIX.as_bytes(), chamber.leveraged_farm.key().as_ref()],
        bump = chamber.bump,
        has_one = authority,
    )]
    pub chamber: Box<Account<'info, state::Chamber>>,

    /// Chamber authority.
    #[account(mut, seeds = [utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(), chamber.key().as_ref()], bump = chamber.authority_bump)]
    pub authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub clock_sysvar: Sysvar<'info, Clock>,
    pub rent_sysvar: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    /*
    /// Accounts expected by Tulip:

    /// 0.
    #[account(mut)]
    pub chamber_farm: UncheckedAccount<'info>,

    /// 1.
    #[account(mut)]
    pub chamber_farm_obligation_vault_0: UncheckedAccount<'info>,

    /// 2.
    #[account(mut)]
    pub chamber_farm_obligation_vault_1: UncheckedAccount<'info>,

    /// 3.
    #[account(mut)]
    pub leveraged_farm: UncheckedAccount<'info>,

    /// 4.
    pub vault_program: UncheckedAccount<'info>,

    /// 5.
    #[account(mut)]
    pub authority_token_account_0: UncheckedAccount<'info>,

    /// 6.
    #[account(mut)]
    pub authority_token_account_1: UncheckedAccount<'info>,

    /// 7.
    #[account(mut)]
    pub vault_pda_account: UncheckedAccount<'info>,

    /// 8.
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,

    /// 9.
    #[account(mut)]
    pub lp_token_account: UncheckedAccount<'info>,

    /// 10.
    #[account(mut)]
    pub chamber_balance_account_0: UncheckedAccount<'info>,

    /// 11.
    #[account(mut)]
    pub chamber_balance_account_1: UncheckedAccount<'info>,

    /// 12.
    pub stake_program: UncheckedAccount<'info>,

    /// 13.
    #[account(mut)]
    pub pool_id: UncheckedAccount<'info>,

    /// 14.
    #[account(mut)]
    pub pool_authority: UncheckedAccount<'info>,

    /// 15.
    #[account(mut)]
    pub vault_info_account: UncheckedAccount<'info>,

    /// 16.
    #[account(mut)]
    pub pool_lp_token_account: UncheckedAccount<'info>,

    /// 17.
    #[account(mut)]
    pub reward_a_token_account: UncheckedAccount<'info>,

    /// 18.
    #[account(mut)]
    pub pool_reward_a_token_account: UncheckedAccount<'info>,

    /// 19.
    #[account(mut)]
    pub reward_b_token_account: UncheckedAccount<'info>,

    /// 20.
    #[account(mut)]
    pub pool_reward_b_token_account: UncheckedAccount<'info>,

    /// 21.
    #[account(mut)]
    pub chamber_balance_metadata_0: UncheckedAccount<'info>,

    /// 22.
    #[account(mut)]
    pub chamber_balance_metadata_1: UncheckedAccount<'info>,

    /// 23.
    pub lending_market: UncheckedAccount<'info>,

    /// 24.
    #[account(mut)]
    pub chamber_farm_obligation_0: UncheckedAccount<'info>,

    /// 25.
    #[account(mut)]
    pub chamber_farm_obligation_1: UncheckedAccount<'info>,

    /// 26.
    pub lending_market_authority: UncheckedAccount<'info>,

    /// 27.
    pub lending_program: UncheckedAccount<'info>,

    /// 28.
    pub levfarm_program: UncheckedAccount<'info>,
    */
}
