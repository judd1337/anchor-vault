use anchor_lang::{
    prelude::*, 
    system_program::{transfer, Transfer},
};

declare_id!("8QR5DCLGkiNbQD5iWZXNmoUSj31ZW1Znnfw5VmX8z4WB");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        //msg!("Greetings from: {:?}", ctx.program_id);
        //Ok(())
        ctx.accounts.initialize(ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    // TODO: Call the close instruction here
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8, // vault system accounts bump
    pub state_bump: u8, // vault state bump
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = VaultState::INIT_SPACE+8,
        seeds = [b"state", signer.key().as_ref()],
        bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        seeds=[vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: InitializeBumps) -> Result<()> {
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;
        msg!("Vault State PDA: {}", self.vault_state.key());
        msg!("Signer: {}", self.signer.key());
        msg!("Vault Bump: {}", self.vault_state.vault_bump);
        Ok(())
    }
}

// Alternative solution only
/* 
pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.vault_state.vault_bump = ctx.bumps.vault;
    ctx.accounts.vault_state.state_bump = ctx.bumps.vault_state;
    Ok(())
}*/

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    
    #[account(
        mut,
        seeds = [vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let system_program = self.system_program.to_account_info();
        
        let accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(system_program, accounts);

        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}


#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    
    #[account(
        mut,
        seeds = [vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        msg!("Vault State PDA: {}", self.vault_state.key());
        msg!("Signer: {}", self.signer.key());
        msg!("Vault Bump: {}", self.vault_state.vault_bump);
        let system_program = self.system_program.to_account_info();
        
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.signer.to_account_info(),
        };

        let seeds = &[
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(system_program, accounts, signer_seeds);
        //let cpi_ctx = CpiContext::new(system_program, accounts);

        assert!(amount <= self.vault.lamports());
        transfer(cpi_ctx, amount)?; // transfer(cpi_ctx, self.vault.lamports())?
        Ok(())
    }
}

/*

// Finish the close() instruction as homework
#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    
    #[account(
        seeds = [vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let system_program = self.system_program.to_account_info();
        
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.signer.to_account_info(),
        };

        let seeds = &[
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(system_program, accounts, signer_seeds);
        //let cpi_ctx = CpiContext::new(system_program, accounts);

        //assert!(amount <= self.vault.lamports());
        transfer(cpi_ctx, amount)?; // transfer(cpi_ctx, self.vault.lamports())?
        Ok(())
    }
}
*/