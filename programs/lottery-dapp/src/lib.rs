use anchor_lang::prelude::*;
use rand::{distributions::Alphanumeric , Rng};




//tasks


/*
    - generate code [x]
    - generates tickets [x]
    - create init users []
    - assign tickets to users []
    - init timer []
    - reward distribution []
*/















declare_id!("9E3vbRyXQ5hXzoJTmDmg25jvDf2kx7MipnPQmGN59Qft");

#[program]
pub mod lottery_dapp {

    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket_acc;
        ticket.price = 0.25;
        let i = 0;
        for i in 0..200{
            ticket.ticket_code[i].to_string();
        }
        ticket.ticket_code[i] = " ".to_string();
        Ok(())
    }

    pub fn generate_code(ctx : Context<Genacc>) -> Result<()>{
        let code_gen = &mut ctx.accounts.rand_acc;
        code_gen.code = (0..200)
        .map(|_| {
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect()
        })
        .collect();
        Ok(())
    }

    pub fn ticket_gen(ctx : Context<Ticketacc> ,) -> Result<()>{
        let ticket = &mut ctx.accounts.ticket_acc;
        let rand = &mut ctx.accounts.rand_acc;
        let i = 0;
        for i in 0..200{
            rand.code[i].to_string();
        }
        ticket.ticket_code[i] = rand.code[i].to_string();
        ticket.price = 0.25;
        Ok(())
    }


}

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(
        init,
        payer = admin,
        space = Ticket::INIT_SPACE
    )]
    pub ticket_acc : Account<'info , Ticket>,

    #[account(
        init,
        payer = admin,
        space = Ticket::INIT_SPACE
    )]
    pub timer_acc : Account<'info , Timer>,
    
    #[account(
        init,
        payer = admin,
        space = Randcode::INIT_SPACE
    )]
    pub rand_acc : Account<'info , Randcode>,

    #[account(mut)]
    pub admin : Signer<'info>,
    pub system_program : Program<'info , System>
}


#[derive(Accounts)]
pub struct Genacc<'info>{
    #[account(
        init,
        payer = admin,
        space = Randcode::INIT_SPACE
    )]
    pub rand_acc : Account<'info , Randcode>,
    #[account(mut)]
    pub admin : Signer<'info>,
    pub system_program : Program<'info , System>
}

#[derive(Accounts)]
pub struct Ticketacc<'info>{
    #[account(
        init,
        payer = admin,
        space = Ticket::INIT_SPACE
    )]
    pub ticket_acc : Account<'info , Ticket>,

    #[account(
        init,
        payer = admin,
        space = Ticket::INIT_SPACE
    )]
    pub rand_acc : Account<'info , Randcode>,
    #[account(mut)]
    pub admin : Signer<'info>,
    pub system_program : Program<'info , System>
}



#[account]
#[derive(InitSpace)]
pub struct Ticket{
    #[max_len(32 , 200)]
    pub ticket_code : Vec<String>,
    pub price : f32
}

#[account]
#[derive(InitSpace)]
pub struct Timer{
    pub duration : u64
}


// random code account
#[account]
#[derive(InitSpace)]
pub struct Randcode{
    #[max_len(32 , 200)]
    pub code : Vec<String> // generated codes goes to the vector
}


//all participating users
#[account]
#[derive(InitSpace)]
pub struct Participates{
    #[max_len(200)]
    pub keys : Vec<Pubkey>,
    #[max_len(30)]
    pub code : String
}


//pool account where users money is in
#[account]
#[derive(InitSpace)]
pub struct Pool{
    pub admin : Pubkey,
    pub balance : u64,
}

// user account data
#[account]
#[derive(InitSpace)]
pub struct User{
    pub key : Pubkey,
    pub ticket : Ticket,
    pub iswinner : bool,
}
