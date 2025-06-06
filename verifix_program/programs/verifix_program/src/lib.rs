use anchor_land::prelude::*;

declare_id!("YourProgramid");


#[program]
pub mod verifix_program{
    use super::*;

    pub fn store_hash(ctx:Context<StoreHash>,file_name:String,hash:[u8;32])->Result<()>{
        let file_hash=&mut ctx.account.file_hash;
        file_hash.hash=hash;
        Ok(())
    }
}