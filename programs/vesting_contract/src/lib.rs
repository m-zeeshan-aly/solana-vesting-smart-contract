use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("Am4968hFBUrKP1g52Y8injrm9PgMUEKS89A2oF2x14wS"); // Replace with your program ID after deployment.

#[program]
pub mod simple_sol_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let contract_account = &mut ctx.accounts.contract_account;
        contract_account.total_received = 0; 
        contract_account.last_received_timestamp = 0; 
        Ok(())
    }
    pub fn receive_sol(ctx: Context<ReceiveSol>, amount: u64) -> Result<()> {
        let contract_account = &mut ctx.accounts.contract_account;

       
        contract_account.total_received += amount;
        contract_account.last_received_timestamp = Clock::get()?.unix_timestamp;

        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.sender.to_account_info(),
                to: ctx.accounts.contract_account.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn send_back_sol(ctx: Context<SendBackSol>, amount: u64) -> Result<()> {
        let contract_account = &mut ctx.accounts.contract_account;

        let current_time = Clock::get()?.unix_timestamp;
        require!(
            current_time >= contract_account.last_received_timestamp + 10,
            ContractError::LockPeriodNotExpired
        );

        let contract_balance = ctx.accounts.contract_account.to_account_info().lamports();
        require!(contract_balance >= amount, ContractError::InsufficientFunds);

        **ctx.accounts.recipient.to_account_info().try_borrow_mut_lamports()? += amount;
        **ctx.accounts.contract_account.to_account_info().try_borrow_mut_lamports()? -= amount;

        Ok(())
    }
    
    
}

#[account]
pub struct ContractAccount {
    pub total_received: u64, 
    pub last_received_timestamp: i64, 
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8 + 8, // 8 bytes for the discriminator, 8 for total_received, 8 for timestamp.
    )]
    pub contract_account: Account<'info, ContractAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReceiveSol<'info> {
    #[account(mut)]
    pub contract_account: Account<'info, ContractAccount>,
    #[account(mut)]
    pub sender: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendBackSol<'info> {
    #[account(mut)]
    pub contract_account: Account<'info, ContractAccount>,
    #[account(mut)]
    pub recipient: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

#[error_code]
pub enum ContractError {
    #[msg("The lock period has not expired. Please wait for 10 seconds.")]
    LockPeriodNotExpired,
    #[msg("Funds are not enough in the contract.")]
    InsufficientFunds,
}

