use super::TestChamberTulip;
use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
use anchor_spl::{
    associated_token::get_associated_token_address,
    token::spl_token::{
        self,
        state::{Account as TokenAccount, AccountState},
    },
};
use cetra_chamber::{
    accounts as cetra_chamber_accounts, instruction as cetra_chamber_instruction,
    state::UserPosition, utils::derive_user_position,
};
use cetra_program_test::{solana_program_test::*, TestContext};
use solana_sdk::{
    account::Account, instruction::Instruction, program_option::COption, program_pack::Pack,
    pubkey::Pubkey, signature::Keypair, signer::Signer, system_instruction, system_program,
    transaction::Transaction, transport,
};

pub struct TestUser {
    pub wallet: Keypair,
}

impl TestUser {
    pub fn new() -> Self {
        TestUser {
            wallet: Keypair::new(),
        }
    }

    pub async fn fund(
        &self,
        test_context: &mut TestContext,
        payer: &Keypair,
        lamports: u64,
    ) -> transport::Result<()> {
        let tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &payer.pubkey(),
                &self.wallet.pubkey(),
                lamports,
            )],
            Some(&payer.pubkey()),
            &[payer],
            test_context.context.last_blockhash,
        );

        test_context
            .context
            .banks_client
            .process_transaction(tx)
            .await?;

        Ok(())
    }

    pub async fn set_ata_balance(
        &self,
        test_context: &mut TestContext,
        mint: &Pubkey,
        amount: u64,
    ) -> Result<(), BanksClientError> {
        let (mut token_account, user_ata) = self.get_ata(test_context, mint).await?;

        token_account.amount = amount;

        let mut data: Vec<u8> = vec![0u8; TokenAccount::LEN];
        token_account.pack_into_slice(&mut data);

        let account = Account {
            lamports: test_context.get_rent().await.minimum_balance(data.len()),
            data,
            owner: spl_token::id(),
            executable: false,
            rent_epoch: 0,
        };

        test_context.context.set_account(&user_ata, &account.into());

        Ok(())
    }

    pub async fn add_ata_balance(
        &self,
        test_context: &mut TestContext,
        mint: &Pubkey,
        amount: u64,
    ) -> Result<(), BanksClientError> {
        let (token_account, _) = self.get_ata(test_context, mint).await?;
        self.set_ata_balance(test_context, mint, token_account.amount + amount)
            .await?;
        Ok(())
    }

    pub async fn initialize_user_position(
        &self,
        test_context: &mut TestContext,
        test_chamber_tulip: &TestChamberTulip,
        base_amount: u64,
        quote_amount: u64,
    ) -> Result<(), BanksClientError> {
        let (user_position, bump) =
            derive_user_position(&self.wallet.pubkey(), &test_chamber_tulip.pubkey);

        let user_base_ata = get_associated_token_address(
            &self.wallet.pubkey(),
            &test_chamber_tulip.farm_config.base_token_mint,
        );
        let user_quote_ata = get_associated_token_address(
            &self.wallet.pubkey(),
            &test_chamber_tulip.farm_config.quote_token_mint,
        );

        let accounts = cetra_chamber_accounts::InitializeUserPosition {
            chamber: test_chamber_tulip.pubkey,
            user_position,
            user_base_ata,
            user_quote_ata,
            chamber_base_ata: test_chamber_tulip.get_base_ata(),
            chamber_quote_ata: test_chamber_tulip.get_quote_ata(),
            payer: self.wallet.pubkey(),
            token_program: spl_token::id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        let data = cetra_chamber_instruction::InitializeUserPosition {
            bump,
            base_amount,
            quote_amount,
        }
        .data();

        let tx = Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: cetra_chamber::id(),
                data,
                accounts,
            }],
            Some(&self.wallet.pubkey()),
            &[&self.wallet],
            test_context.context.last_blockhash,
        );

        Ok(test_context.process_transaction(tx).await.unwrap())
    }

    pub async fn load_user_position(
        &self,
        test_context: &mut TestContext,
        test_chamber_tulip: &TestChamberTulip,
    ) -> Result<UserPosition, BanksClientError> {
        let (user_position, _) =
            derive_user_position(&self.wallet.pubkey(), &test_chamber_tulip.pubkey);

        let account = test_context
            .context
            .banks_client
            .get_account(user_position)
            .await?
            .unwrap();

        Ok(UserPosition::try_deserialize(&mut account.data.as_ref()).unwrap())
    }

    pub async fn get_ata(
        &self,
        test_context: &mut TestContext,
        mint: &Pubkey,
    ) -> Result<(TokenAccount, Pubkey), BanksClientError> {
        let user_ata = get_associated_token_address(&self.wallet.pubkey(), mint);

        let account = test_context
            .context
            .banks_client
            .get_account(user_ata)
            .await?
            .unwrap();
        let token_account = TokenAccount::unpack(&account.data).unwrap();

        Ok((token_account, user_ata))
    }

    pub async fn create_ata(
        &self,
        test_context: &mut TestContext,
        mint: &Pubkey,
        amount: u64,
    ) -> Result<Pubkey, BanksClientError> {
        let user_ata = get_associated_token_address(&self.wallet.pubkey(), mint);
        let rent_lamports = test_context
            .get_rent()
            .await
            .minimum_balance(TokenAccount::LEN);
        let is_native = if mint == &spl_token::native_mint::id() {
            COption::Some(amount)
        } else {
            COption::None
        };

        let token_account = TokenAccount {
            mint: mint.clone(),
            owner: self.wallet.pubkey(),
            amount,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native,
            delegated_amount: 0,
            close_authority: COption::None,
        };

        let mut data: Vec<u8> = vec![0u8; TokenAccount::LEN];
        token_account.pack_into_slice(&mut data);

        let account = Account {
            lamports: if is_native.is_some() {
                rent_lamports + amount
            } else {
                rent_lamports
            },
            data,
            owner: spl_token::id(),
            executable: false,
            rent_epoch: 0,
        };

        test_context.context.set_account(&user_ata, &account.into());

        Ok(user_ata)
    }
}
