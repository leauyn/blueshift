use anchor_lang::prelude::*;

declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    // 使用 Deposit 上下文来创建和存入
    pub fn deposit(ctx: Context<Depoist>, amount: u64) -> Result<()> {
        // 1. 检查存入金额是否大于0 (账户的租金由 `init` 约束自动处理)
        require_gt!(amount, 0, VaultError::InvalidAmount);

        // 2. 执行从用户到金库的转账
        // 因为 `init` 约束已经创建了 vault 账户并支付了租金,
        // 我们只需要把额外的 amount 转进去
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.vault.to_account_info()
            }
        );
        anchor_lang::system_program::transfer(cpi_context, amount)?;
        Ok(())
    }

    // 使用 Withdraw 上下文来取出和关闭
    // 这个函数不再需要 amount 参数，因为它会取出所有余额并关闭账户
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        // 不需要任何代码！
        // `#[account(mut, close = signer)]` 约束已经为你处理了所有事情：
        // 1. 将 vault 账户中的所有 lamports (包括租金) 安全地转给 signer。
        // 2. 关闭 vault 账户。
        Ok(())
    }

}

// 用于存款的账户结构
#[derive(Accounts)]
pub struct Depoist<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init, // <-- 关键点: 第一次调用时创建账户
        payer = signer, // <-- 指定创建账户的付款人
        space = 8, // <-- 为 vault 账户分配空间 (8字节用于discriminator)
        seeds = [b"vault", signer.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

        #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump,
        close = signer, // <-- 关键点: 交易结束后将账户余额转给 signer 并关闭账户
    )]
    pub vault: Account<'info, VaultState>,
}

#[account]
pub struct VaultState {}

#[error_code]
pub enum VaultError{
    #[msg("Vault already exists")]
    VaultAlreadyExists,
    #[msg("Invalid amount")]
    InvalidAmount,
}

