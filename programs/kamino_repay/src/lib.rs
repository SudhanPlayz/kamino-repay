use anchor_lang::{prelude::*, solana_program::instruction::Instruction};

declare_id!("BR6fikkSmQxwzPboVMZVBuN8C9VjTiUpEBd8DrJz4NVU");

#[program]
pub mod kamino_repay {
    use anchor_lang::solana_program::program::invoke;

    use super::*;

    pub fn execute_kamino_repay(
        ctx: Context<ExecuteKaminoRepay>,
        flash_borrow_amount: u64,
        withdraw_amount: u64,
        flash_repay_amount: u64,
        swap_amount: u64,
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
                flash_borrow_amount,
            ),
            &accounts[6..18].to_vec(),
        )?;

        // Step 3: Refresh USDC Reserve
        invoke(
            &refresh_reserve_instruction(
                &ctx.accounts.kamino_lending_program.key(),
                &accounts[18..24],
            ),
            &accounts[18..24].to_vec(),
        )?;

        // Step 4: Refresh JitoSOL Reserve
        invoke(
            &refresh_reserve_instruction(
                &ctx.accounts.kamino_lending_program.key(),
                &accounts[24..30],
            ),
            &accounts[24..30].to_vec(),
        )?;

        // Step 5: Refresh Obligation
        invoke(
            &refresh_obligation_instruction(
                &ctx.accounts.kamino_lending_program.key(),
                &accounts[30..34],
            ),
            &accounts[30..34].to_vec(),
        )?;

        // Step 6: Repay Obligation Liquidity
        invoke(
            &repay_obligation_liquidity_instruction(
                &ctx.accounts.kamino_lending_program.key(),
                &accounts[34..44],
                flash_borrow_amount,
            ),
            &accounts[34..44].to_vec(),
        )?;

        // Step 7: Refresh JitoSOL Reserve
        invoke(
            &refresh_reserve_instruction(
                &ctx.accounts.kamino_lending_program.key(),
                &accounts[44..50],
            ),
            &accounts[44..50].to_vec(),
        )?;

        // Step 8: Refresh USDC Reserve
        invoke(
            &refresh_reserve_instruction(
                &ctx.accounts.kamino_lending_program.key(),
                &accounts[50..56],
            ),
            &accounts[50..56].to_vec(),
        )?;

        // Step 9: Refresh Obligation
        invoke(
            &refresh_obligation_instruction(
                &ctx.accounts.kamino_lending_program.key(),
                &accounts[56..59],
            ),
            &accounts[56..59].to_vec(),
        )?;

        // Step 10: Refresh Obligation Farms for Reserve
        invoke(
            &refresh_obligation_farms_for_reserve_instruction(
                &ctx.accounts.kamino_lending_program.key(),
                &accounts[59..69],
            ),
            &accounts[59..69].to_vec(),
        )?;

        // Step 11: Withdraw Obligation Collateral and Redeem Reserve Collateral
        invoke(
            &withdraw_obligation_collateral_and_redeem_reserve_collateral_instruction(
                &ctx.accounts.kamino_lending_program.key(),
                &accounts[69..83],
                withdraw_amount,
            ),
            &accounts[69..83].to_vec(),
        )?;

        // Step 12: Refresh Obligation Farms for Reserve (again)
        invoke(
            &refresh_obligation_farms_for_reserve_instruction(
                &ctx.accounts.kamino_lending_program.key(),
                &accounts[83..93],
            ),
            &accounts[83..93].to_vec(),
        )?;

        // Step 13: Jupiter swap
        invoke(
            &jupiter_swap_instruction(
                &ctx.accounts.jupiter_program.key(),
                &accounts[93..101],
                swap_amount,
            ),
            &accounts[93..101].to_vec(),
        )?;

        // Step 14: Flash Repay Reserve Liquidity
        invoke(
            &flash_repay_reserve_liquidity_instruction(
                &ctx.accounts.kamino_lending_program.key(),
                &accounts[101..113],
                flash_repay_amount,
            ),
            &accounts[101..113].to_vec(),
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExecuteKaminoRepay<'info> {
    pub associated_token_program: AccountInfo<'info>, // ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL
    pub kamino_lending_program: AccountInfo<'info>,   // KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD
    pub jupiter_program: AccountInfo<'info>,          // JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4
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
        data: vec![], // CreateIdempotent doesn't require additional data
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
            let mut data = vec![0x1];
            data.extend_from_slice(&amount.to_le_bytes());
            data
        },
    }
}

fn refresh_reserve_instruction(
    kamino_lending_program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Instruction {
    Instruction {
        program_id: *kamino_lending_program_id,
        accounts: vec![
            AccountMeta::new(accounts[0].key(), false),
            AccountMeta::new_readonly(accounts[1].key(), false),
            AccountMeta::new_readonly(accounts[2].key(), false),
            AccountMeta::new_readonly(accounts[3].key(), false),
            AccountMeta::new_readonly(accounts[4].key(), false),
            AccountMeta::new_readonly(accounts[5].key(), false),
        ],
        data: vec![0x2],
    }
}

fn refresh_obligation_instruction(
    kamino_lending_program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Instruction {
    Instruction {
        program_id: *kamino_lending_program_id,
        accounts: vec![
            AccountMeta::new_readonly(accounts[0].key(), false),
            AccountMeta::new(accounts[1].key(), false),
            AccountMeta::new(accounts[2].key(), false),
            AccountMeta::new(accounts[3].key(), false),
        ],
        data: vec![0x3],
    }
}

fn repay_obligation_liquidity_instruction(
    kamino_lending_program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> Instruction {
    Instruction {
        program_id: *kamino_lending_program_id,
        accounts: vec![
            AccountMeta::new(accounts[0].key(), true),
            AccountMeta::new(accounts[1].key(), false),
            AccountMeta::new_readonly(accounts[2].key(), false),
            AccountMeta::new(accounts[3].key(), false),
            AccountMeta::new(accounts[4].key(), false),
            AccountMeta::new(accounts[5].key(), false),
            AccountMeta::new(accounts[6].key(), false),
            AccountMeta::new_readonly(accounts[7].key(), false),
            AccountMeta::new_readonly(accounts[8].key(), false),
            AccountMeta::new(accounts[9].key(), false),
        ],
        data: {
            let mut data = vec![0x4];
            data.extend_from_slice(&amount.to_le_bytes());
            data
        },
    }
}

fn refresh_obligation_farms_for_reserve_instruction(
    kamino_lending_program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Instruction {
    Instruction {
        program_id: *kamino_lending_program_id,
        accounts: vec![
            AccountMeta::new(accounts[0].key(), true),
            AccountMeta::new(accounts[1].key(), false),
            AccountMeta::new(accounts[2].key(), false),
            AccountMeta::new(accounts[3].key(), false),
            AccountMeta::new(accounts[4].key(), false),
            AccountMeta::new(accounts[5].key(), false),
            AccountMeta::new_readonly(accounts[6].key(), false),
            AccountMeta::new_readonly(accounts[7].key(), false),
            AccountMeta::new_readonly(accounts[8].key(), false),
            AccountMeta::new_readonly(accounts[9].key(), false),
        ],
        data: vec![0x5],
    }
}

fn withdraw_obligation_collateral_and_redeem_reserve_collateral_instruction(
    kamino_lending_program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> Instruction {
    Instruction {
        program_id: *kamino_lending_program_id,
        accounts: vec![
            AccountMeta::new(accounts[0].key(), true),
            AccountMeta::new(accounts[1].key(), false),
            AccountMeta::new_readonly(accounts[2].key(), false),
            AccountMeta::new(accounts[3].key(), false),
            AccountMeta::new(accounts[4].key(), false),
            AccountMeta::new(accounts[5].key(), false),
            AccountMeta::new(accounts[6].key(), false),
            AccountMeta::new(accounts[7].key(), false),
            AccountMeta::new(accounts[8].key(), false),
            AccountMeta::new(accounts[9].key(), false),
            AccountMeta::new_readonly(accounts[10].key(), false),
            AccountMeta::new_readonly(accounts[11].key(), false),
            AccountMeta::new_readonly(accounts[12].key(), false),
            AccountMeta::new_readonly(accounts[13].key(), false),
        ],
        data: {
            let mut data = vec![0x6];
            data.extend_from_slice(&amount.to_le_bytes());
            data
        },
    }
}

fn flash_repay_reserve_liquidity_instruction(
    kamino_lending_program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> Instruction {
    Instruction {
        program_id: *kamino_lending_program_id,
        accounts: vec![
            AccountMeta::new(accounts[0].key(), true),
            AccountMeta::new(accounts[1].key(), false),
            AccountMeta::new_readonly(accounts[2].key(), false),
            AccountMeta::new(accounts[3].key(), false),
            AccountMeta::new(accounts[4].key(), false),
            AccountMeta::new(accounts[5].key(), false),
            AccountMeta::new(accounts[6].key(), false),
            AccountMeta::new(accounts[7].key(), false),
            AccountMeta::new_readonly(accounts[8].key(), false),
            AccountMeta::new_readonly(accounts[9].key(), false),
            AccountMeta::new_readonly(accounts[10].key(), false),
            AccountMeta::new_readonly(accounts[11].key(), false),
        ],
        data: {
            let mut data = vec![0x7];
            data.extend_from_slice(&amount.to_le_bytes());
            data
        },
    }
}

fn jupiter_swap_instruction(
    jupiter_program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> Instruction {
    Instruction {
        program_id: *jupiter_program_id,
        accounts: vec![
            // Removed the other accounts that were not essential for the basic swap functionality.
            AccountMeta::new_readonly(accounts[0].key(), false), // Token Program
            AccountMeta::new(accounts[1].key(), false),          // Program Authority
            AccountMeta::new(accounts[2].key(), true),           // User Transfer Authority
            AccountMeta::new(accounts[3].key(), false),          // Source Token Account
            AccountMeta::new(accounts[4].key(), false),          // Destination Token Account
            AccountMeta::new_readonly(accounts[5].key(), false), // Source Mint
            AccountMeta::new_readonly(accounts[6].key(), false), // Destination Mint
            AccountMeta::new_readonly(accounts[7].key(), false), // Platform Fee Account
        ],
        data: {
            let mut data = vec![0x8];
            data.extend_from_slice(&amount.to_le_bytes());
            data
        },
    }
}
