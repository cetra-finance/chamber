use crate::{
    handler::{deposit_vault_raydium_tulip_levfarm, DepositVaultRaydiumTulipLevfarmAccounts},
    state, ChamberError, SettleChamberPosition2,
};
use anchor_lang::prelude::*;

impl<'c, 'info> SettleChamberPosition2<'info> {
    pub fn process(
        &mut self,
        remaining_accounts: &'c [AccountInfo<'info>],
        nonce_0: u8,
        nonce_1: u8,
        meta_nonce_0: u8,
        meta_nonce_1: u8,
    ) -> Result<()> {
        // TODO: Extend protocols support
        if self.chamber.protocol_type != state::ProtocolType::Tulip {
            return Err(ChamberError::UnsupportedProtocol.into());
        }

        // TODO: Support more AMM's

        deposit_vault_raydium_tulip_levfarm(
            Box::new(DepositVaultRaydiumTulipLevfarmAccounts {
                chamber: &self.chamber,
                chamber_authority: &self.authority,
                chamber_farm: &remaining_accounts[0],
                chamber_farm_obligation_vault: &remaining_accounts[1],
                leveraged_farm: &remaining_accounts[3],
                vault_program: &remaining_accounts[4],
                authority_token_account: &remaining_accounts[5],
                vault_pda_account: &remaining_accounts[7],
                vault: &remaining_accounts[8],
                lp_token_account: &remaining_accounts[9],
                chamber_balance_account: &remaining_accounts[10],
                system_program: &self.system_program,
                stake_program: &remaining_accounts[12],
                pool_id: &remaining_accounts[13],
                pool_authority: &remaining_accounts[14],
                vault_info_account: &remaining_accounts[15],
                pool_lp_token_account: &remaining_accounts[16],
                reward_a_token_account: &remaining_accounts[17],
                pool_reward_a_token_account: &remaining_accounts[18],
                reward_b_token_account: &remaining_accounts[19],
                pool_reward_b_token_account: &remaining_accounts[20],
                clock_sysvar: &self.clock_sysvar,
                rent_sysvar: &self.rent_sysvar,
                token_program_id: &self.token_program,
                chamber_balance_metadata: &remaining_accounts[21],
                lending_market: &remaining_accounts[23],
                chamber_farm_obligation: &remaining_accounts[24],
                lending_market_authority: &remaining_accounts[26],
                lending_program: &remaining_accounts[27],
                levfarm_program: &remaining_accounts[28],
            }),
            nonce_0,
            meta_nonce_0,
            0,
        )?;

        deposit_vault_raydium_tulip_levfarm(
            Box::new(DepositVaultRaydiumTulipLevfarmAccounts {
                chamber: &self.chamber,
                chamber_authority: &self.authority,
                chamber_farm: &remaining_accounts[0],
                chamber_farm_obligation_vault: &remaining_accounts[2],
                leveraged_farm: &remaining_accounts[3],
                vault_program: &remaining_accounts[4],
                authority_token_account: &remaining_accounts[6],
                vault_pda_account: &remaining_accounts[7],
                vault: &remaining_accounts[8],
                lp_token_account: &remaining_accounts[9],
                chamber_balance_account: &remaining_accounts[11],
                system_program: &self.system_program,
                stake_program: &remaining_accounts[12],
                pool_id: &remaining_accounts[13],
                pool_authority: &remaining_accounts[14],
                vault_info_account: &remaining_accounts[15],
                pool_lp_token_account: &remaining_accounts[16],
                reward_a_token_account: &remaining_accounts[17],
                pool_reward_a_token_account: &remaining_accounts[18],
                reward_b_token_account: &remaining_accounts[19],
                pool_reward_b_token_account: &remaining_accounts[20],
                clock_sysvar: &self.clock_sysvar,
                rent_sysvar: &self.rent_sysvar,
                token_program_id: &self.token_program,
                chamber_balance_metadata: &remaining_accounts[22],
                lending_market: &remaining_accounts[23],
                chamber_farm_obligation: &remaining_accounts[25],
                lending_market_authority: &remaining_accounts[26],
                lending_program: &remaining_accounts[27],
                levfarm_program: &remaining_accounts[28],
            }),
            nonce_1,
            meta_nonce_1,
            1,
        )?;

        Ok(())
    }
}
