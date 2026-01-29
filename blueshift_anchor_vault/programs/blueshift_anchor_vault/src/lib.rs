use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

// 确保 Program ID 是 Blueprint 平台要求的固定 ID
declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    // 存款指令
    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        // 1. 验证金库为空
        require_eq!(ctx.accounts.vault.lamports(), 0, VaultError::VaultAlreadyExists);

        // 2. 确保存款金额超过免租金最低限额
        require_gt!(amount, Rent::get()?.minimum_balance(0), VaultError::InvalidAmount);

        // 3. 使用 CPI 调用系统程序，将 lamports 从签名者转移到金库
        let cpi_accounts = Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };
        let cpi_context = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
        transfer(cpi_context, amount)?;

        Ok(())
    }

    // 提款指令
    pub fn withdraw(ctx: Context<VaultAction>) -> Result<()> {
        // 1. 验证保险库中是否有 lamports
        require_gt!(ctx.accounts.vault.lamports(), 0, VaultError::NoLamportsToWithdraw);

        // 2. 获取金库的总余额
        let amount_to_withdraw = ctx.accounts.vault.lamports();

        // 3. 【关键修复】创建 PDA 签名者种子
        let signer_key = ctx.accounts.signer.key();
        let bump_seed = [ctx.bumps.vault]; // 创建一个具名变量，延长其生命周期
        let signer_seeds = &[b"vault", signer_key.as_ref(), &bump_seed]; // 引用该具名变量

        // 关键：把“种子集合（外层）”绑定到变量，延长生命周期
        let signer_seeds_arr: &[&[&[u8]]] = &[signer_seeds];

        // 4. 执行 CPI 转账
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.signer.to_account_info(),
        };
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            cpi_accounts,
            signer_seeds_arr
        );
        transfer(cpi_context, amount_to_withdraw)?;
        
        Ok(())
    }
}

// 账户上下文
#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

// 错误码
#[error_code]
pub enum VaultError {
    #[msg("Vault already exists")]
    VaultAlreadyExists,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("No lamports to withdraw from vault")]
    NoLamportsToWithdraw,
}
