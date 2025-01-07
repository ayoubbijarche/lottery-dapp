use anchor_lang::prelude::*;
use rand::prelude::*;

declare_id!("9E3vbRyXQ5hXzoJTmDmg25jvDf2kx7MipnPQmGN59Qft");

#[program]
pub mod lottery_dapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(
        init,
        payer = signer,
        space = Ticket::INIT_SPACE
    )]
    pub ticket_acc : Account<'info , Ticket>,

    #[account(
        init,
        payer = signer,
        space = Ticket::INIT_SPACE
    )]

    pub timer_acc : Account<'info , Timer>,

    #[account(mut)]
    pub signer : Signer<'info>,
    pub system_program : Program<'info , System>
}

#[account]
#[derive(InitSpace)]
pub struct Ticket{
    #[max_len(32)]
    pub ticket_code : String
}

#[account]
#[derive(InitSpace)]
pub struct Timer{

}

#[account]
#[derive(InitSpace)]
pub struct Randcode{
    #[max_len(32 , 200)]
    pub code : Vec<String> // generated codes goes to the vector
}

#[account]
#[derive(InitSpace)]
pub struct Participates{
    #[max_len(200)]
    pub keys : Vec<Pubkey>,
    #[max_len(30)]
    pub code : String
}

#[account]
#[derive(InitSpace)]
pub struct Pool{
    pub admin : Pubkey,
    pub balance : u64,
}


#[account]
#[derive(InitSpace)]
pub struct User{

}
