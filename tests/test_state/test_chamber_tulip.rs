use super::TestUser;
use crate::test_utils::{raydium_raysrm_farm, serum_program, tulip_mint};
use anchor_lang::{prelude::AccountMeta, AccountDeserialize, InstructionData, ToAccountMetas};
use anchor_spl::{
    associated_token::{self, get_associated_token_address},
    token,
};
use cetra_chamber::{
    accounts as cetra_chamber_accounts, instruction as cetra_chamber_instruction,
    state::Chamber,
    utils::{derive_chamber_address, derive_chamber_authority, derive_user_position},
};
use cetra_program_test::{solana_program_test::*, *};
use solana_sdk::{
    compute_budget,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program,
    sysvar::{clock, rent},
    transaction::Transaction,
};
use tulipv2_sdk_common::config::{
    levfarm::{
        ray_solusdc,
        reserves::{
            sol::LIQUIDITY_SUPPLY_TOKEN_ACCOUNT as SOL_LIQUIDITY_SUPPLY_TOKEN_ACCOUNT,
            usdc::LIQUIDITY_SUPPLY_TOKEN_ACCOUNT as USDC_LIQUIDITY_SUPPLY_TOKEN_ACCOUNT,
        },
        LevFarmConfig, BORROW_AUTHORIZER,
    },
    RAYDIUM_LIQUIDITY_V4, RAYDIUM_STAKE_V5,
};
use tulipv2_sdk_levfarm::accounts::{
    derivations::{
        derive_user_farm_address, derive_user_farm_obligation_address,
        derive_user_farm_obligation_vault_address, derive_user_position_info_address,
    },
    Farms,
};

/// TODO: Add support for other AMMs.
pub struct TestChamberTulip {
    pub farm_config: LevFarmConfig,
    pub farm: Farms,
    pub pubkey: Pubkey,
    pub authority: Pubkey,

    /// Depends on underlying AMM.
    pub base_liquidity_supply_token_account: Pubkey,

    /// Depends on underlying AMM.
    pub quote_liquidity_supply_token_account: Pubkey,

    /// Depends on underlying AMM.
    pub market_amm_authority: Pubkey,

    /// Depends on underlying AMM.
    pub market_vault_signer: Pubkey,

    /// Depends on underlying AMM.
    pub vault_pool_authority: Pubkey,

    /// Depends on underlying AMM.
    pub vault_pda: Pubkey,

    /// Depends on underlying AMM.
    pub vault_info: Pubkey,

    /// Depends on underlying AMM.
    pub vault_lp_token_account: Pubkey,

    /// Depends on underlying AMM.
    pub vault_reward_a_token_account: Pubkey,

    /// Depends on underlying AMM.
    pub vault_pool_reward_a_token_account: Pubkey,

    /// Depends on underlying AMM.
    pub vault_reward_b_token_account: Pubkey,

    /// Depends on underlying AMM.
    pub vault_pool_reward_b_token_account: Pubkey,

    /// Depends on underlying AMM.
    pub vault_farm: Pubkey,

    pub bump: u8,
    pub authority_bump: u8,
}

impl TestChamberTulip {
    pub fn new(
        farm_config: LevFarmConfig,
        farm: Farms,
        base_liquidity_supply_token_account: Pubkey,
        quote_liquidity_supply_token_account: Pubkey,
        market_amm_authority: Pubkey,
        market_vault_signer: Pubkey,
        vault_pool_authority: Pubkey,
        vault_pda: Pubkey,
        vault_info: Pubkey,
        vault_lp_token_account: Pubkey,
        vault_reward_a_token_account: Pubkey,
        vault_pool_reward_a_token_account: Pubkey,
        vault_reward_b_token_account: Pubkey,
        vault_pool_reward_b_token_account: Pubkey,
        vault_farm: Pubkey,
    ) -> Self {
        let (pubkey, bump) = derive_chamber_address(&farm_config.account);
        let (authority, authority_bump) = derive_chamber_authority(&pubkey);

        TestChamberTulip {
            farm_config,
            farm,
            pubkey,
            authority,
            base_liquidity_supply_token_account,
            quote_liquidity_supply_token_account,
            market_amm_authority,
            market_vault_signer,
            vault_pool_authority,
            vault_pda,
            vault_info,
            vault_lp_token_account,
            vault_reward_a_token_account,
            vault_pool_reward_a_token_account,
            vault_reward_b_token_account,
            vault_pool_reward_b_token_account,
            vault_farm,
            bump,
            authority_bump,
        }
    }

    pub fn new_sol_usdc_raydium() -> Self {
        Self::new(
            ray_solusdc::get_lev_farm_config(),
            Farms::SolUsdcRayVault,
            SOL_LIQUIDITY_SUPPLY_TOKEN_ACCOUNT,
            USDC_LIQUIDITY_SUPPLY_TOKEN_ACCOUNT,
            ray_solusdc::market_config::AMM_AUTHORITY,
            ray_solusdc::market_config::SERUM_VAULT_SIGNER,
            ray_solusdc::vault_config::POOL_AUTHORITY,
            ray_solusdc::vault_config::PDA,
            ray_solusdc::vault_config::VAULT_INFO_ACCOUNT,
            ray_solusdc::vault_config::LP_TOKEN_ACCOUNT,
            ray_solusdc::vault_config::REWARD_A_TOKEN_ACCOUNT,
            ray_solusdc::vault_config::POOL_REWARD_A_TOKEN_ACCOUNT,
            ray_solusdc::vault_config::REWARD_B_TOKEN_ACCOUNT,
            ray_solusdc::vault_config::POOL_REWARD_B_TOKEN_ACCOUNT,
            raydium_raysrm_farm::id(),
        )
    }

    pub fn derive_chamber_farm(&self) -> (Pubkey, u8) {
        derive_user_farm_address(self.authority, tulipv2_sdk_levfarm::ID, 0, self.farm)
    }

    pub fn derive_chamber_obligation(&self, id: usize) -> (Pubkey, u8) {
        let (chamber_farm, _) = self.derive_chamber_farm();

        derive_user_farm_obligation_address(
            self.authority,
            chamber_farm,
            tulipv2_sdk_levfarm::ID,
            id as u8,
        )
    }

    pub fn derive_chamber_obligation_vault(&self, id: usize) -> (Pubkey, u8) {
        let (chamber_farm, _) = self.derive_chamber_farm();

        derive_user_farm_obligation_vault_address(chamber_farm, tulipv2_sdk_levfarm::ID, id as u8)
    }

    pub fn derive_chamber_position_info(&self, id: usize) -> (Pubkey, u8) {
        let (chamber_farm, _) = self.derive_chamber_farm();

        derive_user_position_info_address(chamber_farm, tulipv2_sdk_levfarm::ID, id as u8)
    }

    pub fn derive_chamber_balance_account(&self, id: usize) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                ray_solusdc::vault_config::VAULT_INFO_ACCOUNT.as_ref(),
                self.derive_chamber_obligation_vault(id).0.as_ref(),
            ],
            &self.farm_config.solfarm_vault_program,
        )
    }

    pub fn derive_chamber_balance_metadata(&self, id: usize) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                self.derive_chamber_balance_account(id).0.as_ref(),
                self.derive_chamber_obligation_vault(id).0.as_ref(),
            ],
            &self.farm_config.solfarm_vault_program,
        )
    }

    pub fn derive_chamber_all(&self, id: usize) -> (Pubkey, Pubkey, Pubkey, Pubkey, Pubkey) {
        (
            self.derive_chamber_obligation(id).0,
            self.derive_chamber_obligation_vault(id).0,
            self.derive_chamber_position_info(id).0,
            self.derive_chamber_balance_account(id).0,
            self.derive_chamber_balance_metadata(id).0,
        )
    }

    pub fn get_base_ata(&self) -> Pubkey {
        get_associated_token_address(&self.authority, &self.farm_config.base_token_mint)
    }

    pub fn get_quote_ata(&self) -> Pubkey {
        get_associated_token_address(&self.authority, &self.farm_config.quote_token_mint)
    }

    pub fn get_tulip_ata(&self, id: usize) -> Pubkey {
        associated_token::get_associated_token_address(
            &self.derive_chamber_obligation_vault(id).0,
            &tulip_mint::id(),
        )
    }

    pub fn get_lp_ata(&self, id: usize) -> Pubkey {
        associated_token::get_associated_token_address(
            &self.derive_chamber_obligation_vault(id).0,
            &self.farm_config.lp_mint,
        )
    }

    pub async fn initialize_chamber(
        &self,
        test_context: &mut TestContext,
        payer: &Keypair,
    ) -> Result<(), BanksClientError> {
        let base_ata = self.get_base_ata();
        let quote_ata = self.get_quote_ata();

        let accounts = cetra_chamber_accounts::InitializeChamber {
            chamber: self.pubkey,
            authority: self.authority,
            base_ata,
            quote_ata,
            base_mint: self.farm_config.base_token_mint,
            quote_mint: self.farm_config.quote_token_mint,
            payer: payer.pubkey(),
            rent_sysvar: rent::id(),
            associated_token_program: associated_token::ID,
            token_program: token::ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        let data = cetra_chamber_instruction::InitializeChamber {
            leveraged_farm: self.farm_config.account,
            bump: self.bump,
            authority_bump: self.authority_bump,
            protocol_type: cetra_chamber::state::ProtocolType::Tulip,
        }
        .data();

        let tx = Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: cetra_chamber::id(),
                data,
                accounts,
            }],
            Some(&payer.pubkey()),
            &[payer],
            test_context.context.last_blockhash,
        );

        Ok(test_context.process_transaction(tx).await.unwrap())
    }

    pub async fn initialize_chamber_strategy(
        &self,
        test_context: &mut TestContext,
        payer: &Keypair,
    ) -> Result<(), BanksClientError> {
        let (chamber_farm, _) = self.derive_chamber_farm();

        let (chamber_farm_obligation_0, chamber_obligation_vault_0, _, _, _) =
            self.derive_chamber_all(0);

        let (chamber_farm_obligation_1, chamber_obligation_vault_1, _, _, _) =
            self.derive_chamber_all(1);

        let raydium_lp_ata_0 = self.get_lp_ata(0);
        let raydium_lp_ata_1 = self.get_lp_ata(1);

        let tulip_ata_0 = self.get_tulip_ata(0);
        let tulip_ata_1 = self.get_tulip_ata(1);

        let mut accounts = cetra_chamber_accounts::InitializeChamberStrategy {
            chamber: self.pubkey,
            authority: self.authority,
            payer: payer.pubkey(),
            clock_sysvar: clock::id(),
            rent_sysvar: rent::id(),
            associated_token_program: associated_token::ID,
            token_program: token::ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        // Add remaining accounts for tulip levfarm and raydium AMM
        accounts.extend(vec![
            AccountMeta::new_readonly(self.farm_config.global, false),
            AccountMeta::new(chamber_farm, false),
            AccountMeta::new(chamber_farm_obligation_0, false),
            AccountMeta::new(chamber_farm_obligation_1, false),
            AccountMeta::new(chamber_obligation_vault_0, false),
            AccountMeta::new(chamber_obligation_vault_1, false),
            AccountMeta::new(self.farm_config.lending_market, false),
            AccountMeta::new_readonly(self.farm_config.account, false),
            AccountMeta::new(raydium_lp_ata_0, false),
            AccountMeta::new(raydium_lp_ata_1, false),
            AccountMeta::new_readonly(self.farm_config.lp_mint, false),
            AccountMeta::new(tulip_ata_0, false),
            AccountMeta::new(tulip_ata_1, false),
            AccountMeta::new_readonly(tulip_mint::id(), false),
            AccountMeta::new_readonly(self.farm_config.lending_program, false),
            AccountMeta::new_readonly(tulipv2_sdk_levfarm::ID, false),
            AccountMeta::new_readonly(self.farm_config.solfarm_vault_program, false),
        ]);

        let data = cetra_chamber_instruction::InitializeChamberStrategy {}.data();

        let tx = Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: cetra_chamber::id(),
                data,
                accounts,
            }],
            Some(&payer.pubkey()),
            &[payer],
            test_context.context.last_blockhash,
        );

        Ok(test_context.process_transaction(tx).await.unwrap())
    }

    pub async fn deposit_chamber(
        &self,
        test_context: &mut TestContext,
        test_user: &TestUser,
        base_amount: u64,
        quote_amount: u64,
    ) -> Result<(), BanksClientError> {
        let (chamber_farm, _) = self.derive_chamber_farm();

        let (chamber_farm_obligation_0, _, chamber_position_info_0, _, _) =
            self.derive_chamber_all(0);

        let (chamber_farm_obligation_1, _, chamber_position_info_1, _, _) =
            self.derive_chamber_all(1);

        let mut accounts = cetra_chamber_accounts::DepositChamber {
            user_position: derive_user_position(&test_user.wallet.pubkey(), &self.pubkey).0,
            chamber: self.pubkey,
            authority: self.authority,
            chamber_base_ata: self.get_base_ata(),
            chamber_quote_ata: self.get_quote_ata(),
            payer: test_user.wallet.pubkey(),
            clock_sysvar: clock::id(),
            rent_sysvar: rent::id(),
            token_program: token::ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        // Add remaining accounts for tulip levfarm and raydium AMM
        accounts.extend(vec![
            AccountMeta::new(chamber_farm, false),
            AccountMeta::new_readonly(self.farm_config.account, false),
            AccountMeta::new(chamber_farm_obligation_0, false),
            AccountMeta::new(chamber_farm_obligation_1, false),
            AccountMeta::new(self.farm_config.base_token_account, false),
            AccountMeta::new(self.farm_config.quote_token_account, false),
            AccountMeta::new(self.farm_config.base_reserve, false),
            AccountMeta::new(self.farm_config.quote_reserve, false),
            AccountMeta::new_readonly(self.farm_config.coin_price_account, false),
            AccountMeta::new_readonly(self.farm_config.pc_price_account, false),
            AccountMeta::new_readonly(self.farm_config.lending_market, false),
            AccountMeta::new_readonly(self.farm_config.lending_market_authority, false),
            AccountMeta::new_readonly(self.farm_config.lending_program, false),
            AccountMeta::new(self.base_liquidity_supply_token_account, false),
            AccountMeta::new(self.quote_liquidity_supply_token_account, false),
            AccountMeta::new(self.farm_config.coin_reserve_fee_receiver, false),
            AccountMeta::new(self.farm_config.pc_reserve_fee_receiver, false),
            AccountMeta::new_readonly(BORROW_AUTHORIZER, false),
            AccountMeta::new_readonly(self.farm_config.lp_price_account, false),
            AccountMeta::new(self.farm_config.vault_account, false),
            AccountMeta::new(chamber_position_info_0, false),
            AccountMeta::new(chamber_position_info_1, false),
            AccountMeta::new_readonly(tulipv2_sdk_levfarm::ID, false),
        ]);

        let data = cetra_chamber_instruction::DepositChamber {
            base_amount,
            quote_amount,
        }
        .data();

        let tx = Transaction::new_signed_with_payer(
            &[
                compute_budget::ComputeBudgetInstruction::request_units(350000, 0),
                Instruction {
                    program_id: cetra_chamber::id(),
                    data,
                    accounts,
                },
            ],
            Some(&test_user.wallet.pubkey()),
            &[&test_user.wallet],
            test_context.context.last_blockhash,
        );

        Ok(test_context.process_transaction(tx).await.unwrap())
    }

    pub async fn settle_chamber_position(
        &self,
        test_context: &mut TestContext,
        payer: &Keypair,
    ) -> Result<(), BanksClientError> {
        let (chamber_farm, _) = self.derive_chamber_farm();

        let (chamber_farm_obligation_0, _, chamber_position_info_0, _, _) =
            self.derive_chamber_all(0);

        let (chamber_farm_obligation_1, _, chamber_position_info_1, _, _) =
            self.derive_chamber_all(1);

        let raydium_lp_ata_0 = self.get_lp_ata(0);
        let raydium_lp_ata_1 = self.get_lp_ata(1);

        let mut accounts = cetra_chamber_accounts::SettleChamberPosition {
            chamber: self.pubkey,
            authority: self.authority,
            payer: payer.pubkey(),
            clock_sysvar: clock::id(),
            rent_sysvar: rent::id(),
            token_program: token::ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        // Add remaining accounts for tulip levfarm and raydium AMM
        accounts.extend(vec![
            AccountMeta::new(self.farm_config.account, false),
            AccountMeta::new(chamber_farm, false),
            AccountMeta::new(chamber_farm_obligation_0, false),
            AccountMeta::new(chamber_farm_obligation_1, false),
            AccountMeta::new(self.farm_config.amm_id, false),
            AccountMeta::new(self.market_amm_authority, false),
            AccountMeta::new(self.farm_config.amm_open_orders, false),
            AccountMeta::new(self.farm_config.amm_quantities_or_target_orders, false),
            AccountMeta::new(self.farm_config.amm_coin_account, false),
            AccountMeta::new(self.farm_config.amm_pc_account, false),
            AccountMeta::new_readonly(serum_program::id(), false),
            AccountMeta::new(self.farm_config.serum_market, false),
            AccountMeta::new(self.farm_config.serum_bids, false),
            AccountMeta::new(self.farm_config.serum_asks, false),
            AccountMeta::new(self.farm_config.serum_event_queue, false),
            AccountMeta::new(self.farm_config.serum_coin_vault, false),
            AccountMeta::new(self.farm_config.serum_pc_vault, false),
            AccountMeta::new(self.market_vault_signer, false),
            AccountMeta::new(self.farm_config.base_token_account, false),
            AccountMeta::new(self.farm_config.quote_token_account, false),
            AccountMeta::new_readonly(self.farm_config.lending_market, false),
            AccountMeta::new_readonly(self.farm_config.lending_market_authority, false),
            AccountMeta::new_readonly(self.farm_config.lending_program, false),
            AccountMeta::new(chamber_position_info_0, false),
            AccountMeta::new(chamber_position_info_1, false),
            AccountMeta::new(self.farm_config.lp_mint, false),
            AccountMeta::new(raydium_lp_ata_0, false),
            AccountMeta::new(raydium_lp_ata_1, false),
            AccountMeta::new_readonly(self.farm_config.lp_price_account, false),
            AccountMeta::new_readonly(tulipv2_sdk_levfarm::ID, false),
            AccountMeta::new_readonly(RAYDIUM_LIQUIDITY_V4, false),
        ]);

        let data = cetra_chamber_instruction::SettleChamberPosition {}.data();

        let tx = Transaction::new_signed_with_payer(
            &[
                compute_budget::ComputeBudgetInstruction::request_units(530000, 0),
                Instruction {
                    program_id: cetra_chamber::id(),
                    data,
                    accounts,
                },
            ],
            Some(&payer.pubkey()),
            &[payer],
            test_context.context.last_blockhash,
        );

        Ok(test_context.process_transaction(tx).await.unwrap())
    }

    pub async fn settle_chamber_position2(
        &self,
        test_context: &mut TestContext,
        payer: &Keypair,
    ) -> Result<(), BanksClientError> {
        let (chamber_farm, _) = self.derive_chamber_farm();

        let (chamber_farm_obligation_0, chamber_farm_obligation_vault_0, _, _, _) =
            self.derive_chamber_all(0);

        let (chamber_balance_account_0, nonce_0) = self.derive_chamber_balance_account(0);
        let (chamber_balance_account_1, nonce_1) = self.derive_chamber_balance_account(1);

        let (chamber_balance_metadata_0, meta_nonce_0) = self.derive_chamber_balance_metadata(0);
        let (chamber_balance_metadata_1, meta_nonce_1) = self.derive_chamber_balance_metadata(1);

        let (chamber_farm_obligation_1, chamber_farm_obligation_vault_1, _, _, _) =
            self.derive_chamber_all(1);

        let raydium_lp_ata_0 = self.get_lp_ata(0);
        let raydium_lp_ata_1 = self.get_lp_ata(1);

        let lp_token_account = associated_token::get_associated_token_address(
            &ray_solusdc::vault_config::PDA,
            &self.farm_config.lp_mint,
        );

        let mut accounts = cetra_chamber_accounts::SettleChamberPosition2 {
            chamber: self.pubkey,
            authority: self.authority,
            payer: payer.pubkey(),
            clock_sysvar: clock::id(),
            rent_sysvar: rent::id(),
            token_program: token::ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        // Add remaining accounts for tulip levfarm and raydium AMM
        accounts.extend(vec![
            AccountMeta::new(chamber_farm, false),
            AccountMeta::new(chamber_farm_obligation_vault_0, false),
            AccountMeta::new(chamber_farm_obligation_vault_1, false),
            AccountMeta::new(self.farm_config.account, false),
            AccountMeta::new_readonly(self.farm_config.solfarm_vault_program, false),
            AccountMeta::new(raydium_lp_ata_0, false),
            AccountMeta::new(raydium_lp_ata_1, false),
            AccountMeta::new(self.vault_pda, false),
            AccountMeta::new(self.farm_config.vault_account, false),
            AccountMeta::new(lp_token_account, false),
            AccountMeta::new(chamber_balance_account_0, false),
            AccountMeta::new(chamber_balance_account_1, false),
            AccountMeta::new_readonly(RAYDIUM_STAKE_V5, false),
            AccountMeta::new(self.vault_farm, false),
            AccountMeta::new(self.vault_pool_authority, false),
            AccountMeta::new(self.vault_info, false),
            AccountMeta::new(self.vault_lp_token_account, false),
            AccountMeta::new(self.vault_reward_a_token_account, false),
            AccountMeta::new(self.vault_pool_reward_a_token_account, false),
            AccountMeta::new(self.vault_reward_b_token_account, false),
            AccountMeta::new(self.vault_pool_reward_b_token_account, false),
            AccountMeta::new(chamber_balance_metadata_0, false),
            AccountMeta::new(chamber_balance_metadata_1, false),
            AccountMeta::new_readonly(self.farm_config.lending_market, false),
            AccountMeta::new(chamber_farm_obligation_0, false),
            AccountMeta::new(chamber_farm_obligation_1, false),
            AccountMeta::new_readonly(self.farm_config.lending_market_authority, false),
            AccountMeta::new_readonly(self.farm_config.lending_program, false),
            AccountMeta::new_readonly(tulipv2_sdk_levfarm::ID, false),
        ]);

        let data = cetra_chamber_instruction::SettleChamberPosition2 {
            nonce_0,
            nonce_1,
            meta_nonce_0,
            meta_nonce_1,
        }
        .data();

        let tx = Transaction::new_signed_with_payer(
            &[
                compute_budget::ComputeBudgetInstruction::request_units(530000, 0),
                Instruction {
                    program_id: cetra_chamber::id(),
                    data,
                    accounts,
                },
            ],
            Some(&payer.pubkey()),
            &[payer],
            test_context.context.last_blockhash,
        );

        Ok(test_context.process_transaction(tx).await.unwrap())
    }

    pub async fn load(&self, test_context: &mut TestContext) -> Result<Chamber, BanksClientError> {
        let account = test_context
            .context
            .banks_client
            .get_account(self.pubkey)
            .await?
            .unwrap();

        Ok(Chamber::try_deserialize(&mut account.data.as_ref()).unwrap())
    }
}
