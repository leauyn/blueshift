use anchor_lang::prelude::*;

declare_id!("AprsZLDgB8uUuij2fz8AWAnybPwmqZGMDJdEQQf6PiZE");

#[program]
pub mod blueshift_anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
