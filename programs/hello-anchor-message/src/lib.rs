use anchor_lang::prelude::*;

declare_id!("5e35KP8GwepMbm7JwHcAxxiGBrZQmJ5qKgaago35UCFk");

#[program]
pub mod hello_anchor_message {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count = 0;
        // let copy = data.clone();
        // base_account.data = data;
        // base_account.data_list.push(copy);
        Ok(())
    }

    pub fn update(ctx: Context<Update>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        // let copy = data.clone();
        // base_account.data = data;
        // base_account.data_list.push(copy);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 64 + 64)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    // pub data: String,
    // pub data_list: Vec<String>,
    pub count: u64,
}
