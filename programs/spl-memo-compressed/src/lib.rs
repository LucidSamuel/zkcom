use anchor_lang::prelude::*;

declare_id!("5dJnfmrWsRwoDNRfeVN7YsKDCbF6K6DikKcLTjWeF1JR");

#[program]
pub mod spl_memo_compressed {
    use super::*;

    pub fn create_memo(ctx: Context<CreateMemo>, compressed_memo: Vec<u8>) -> Result<()> {
        // The memo is already compressed client-side using ZK Compression
        let ix = spl_memo::build_memo(
            &compressed_memo,
            &[&ctx.accounts.signer.key()]
        );
        
        solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.memo_program.to_account_info(),
                ctx.accounts.signer.to_account_info(),
            ]
        )?;

        msg!("ZK-compressed memo stored");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMemo<'info> {
    /// CHECK: This is the SPL Memo program
    pub memo_program: AccountInfo<'info>,
    
    #[account(mut)]
    pub signer: Signer<'info>,
}
