use anchor_lang::prelude::*;

declare_id!("3G7fYQpdrsuRSHPqSGoTBZunmxqLTXYZdrd9VwkUiUUs");


#[program]
pub mod verifx_program{
    use super::*;

    pub fn store_hash(ctx:Context<StoreHash>,file_name:String,hash:[u8;32])->Result<()>{
        let file_hash=&mut ctx.accounts.file_hash;
        file_hash.hash=hash;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(file_name:String)]
pub struct StoreHash<'info>{
    #[account(
        init,
        payer=user,
        space=8+32,
        seeds=[user.key().as_ref(),file_name.as_bytes()],
        bump
    )]

    pub file_hash:Account<'info,FileHash>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program:Program<'info,System>,
}

#[account]
pub struct FileHash{
    pub hash:[u8;32],
}