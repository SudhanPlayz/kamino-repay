use anchor_lang::{prelude::*, solana_program::instruction::Instruction};

declare_id!("BR6fikkSmQxwzPboVMZVBuN8C9VjTiUpEBd8DrJz4NVU");

#[program]
pub mod kamino_repay {
    use anchor_lang::solana_program::program::invoke;

    use super::*;

    pub fn execute_kamino_repay(
        ctx: Context<ExecuteKaminoRepay>,
        amount: u64,
    ) -> Result<()> {
        let accounts = &ctx.remaining_accounts;

        // Step 1: Create Associated Token Account (if necessary)
        invoke(
            &create_associated_token_account_instruction(
                &ctx.accounts.associated_token_program.key(),
                accounts,
            ),
            &[
                accounts[0].clone(),
                accounts[1].clone(),
                accounts[2].clone(),
                accounts[3].clone(),
                accounts[4].clone(),
                accounts[5].clone(),
            ],
        )?;

        // Step 2: Flash Borrow Reserve Liquidity
        invoke(
            &flash_borrow_reserve_liquidity_instruction(
                &ctx.accounts.kamino_lending_program.key(),
                accounts,
                amount,
            ),
            &[
                accounts[6].clone(),
                accounts[7].clone(),
                accounts[8].clone(),
                accounts[9].clone(),
                accounts[10].clone(),
                accounts[11].clone(),
                accounts[12].clone(),
                accounts[13].clone(),
                accounts[14].clone(),
                accounts[15].clone(),
                accounts[16].clone(),
                accounts[17].clone(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExecuteKaminoRepay<'info> {
    pub associated_token_program: AccountInfo<'info>,
    pub kamino_lending_program: AccountInfo<'info>,
}

fn create_associated_token_account_instruction(
    associated_token_program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Instruction {
    Instruction {
        program_id: *associated_token_program_id,
        accounts: vec![
            AccountMeta::new(accounts[0].key(), false),
            AccountMeta::new(accounts[1].key(), false),
            AccountMeta::new(accounts[2].key(), true),
            AccountMeta::new_readonly(accounts[3].key(), false),
            AccountMeta::new_readonly(accounts[4].key(), false),
            AccountMeta::new(accounts[5].key(), true),
        ],
        data: vec![],  // CreateIdempotent doesn't require additional data
    }
}

fn flash_borrow_reserve_liquidity_instruction(
    kamino_lending_program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> Instruction {
    Instruction {
        program_id: *kamino_lending_program_id,
        accounts: vec![
            AccountMeta::new(accounts[6].key(), true),
            AccountMeta::new(accounts[7].key(), false),
            AccountMeta::new_readonly(accounts[8].key(), false),
            AccountMeta::new(accounts[9].key(), false),
            AccountMeta::new(accounts[10].key(), false),
            AccountMeta::new(accounts[11].key(), false),
            AccountMeta::new(accounts[12].key(), false),
            AccountMeta::new(accounts[13].key(), false),
            AccountMeta::new_readonly(accounts[14].key(), false),
            AccountMeta::new_readonly(accounts[15].key(), false),
            AccountMeta::new_readonly(accounts[16].key(), false),
            AccountMeta::new_readonly(accounts[17].key(), false),
        ],
        data: {
            let mut data = vec![0x0B];  // Assuming 0x0B is the instruction code for FlashBorrowReserveLiquidity
            data.extend_from_slice(&amount.to_le_bytes());
            data
        },
    }
}