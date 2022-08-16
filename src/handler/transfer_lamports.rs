use anchor_lang::{prelude::*, system_program};

pub struct TransferLamportsAccounts<'c, 'info> {
    pub from: &'c AccountInfo<'info>,
    pub to: &'c AccountInfo<'info>,
    pub system_program: &'c Program<'info, System>,
}

#[inline(always)]
pub fn transfer_lamports<'c, 'info>(
    accounts: Box<TransferLamportsAccounts<'c, 'info>>,
    amount: u64,
) -> Result<()> {
    let cpi_accounts = system_program::Transfer {
        from: accounts.from.to_account_info(),
        to: accounts.to.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(accounts.system_program.to_account_info(), cpi_accounts);
    system_program::transfer(cpi_ctx, amount)
}
