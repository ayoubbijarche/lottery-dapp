use anchor_lang::prelude::*;


declare_id!("9E3vbRyXQ5hXzoJTmDmg25jvDf2kx7MipnPQmGN59Qft");

#[program]
pub mod lottery_dapp {

    use anchor_lang::solana_program::{program::invoke, system_instruction};

    use super::*;
    pub fn initialize(ctx : Context<Initialize> , price : u64 , end_time_stamp : i64)-> Result<()>{
        let lottery = &mut ctx.accounts.lottery;
        lottery.authority = lottery.authority.key();
        lottery.price = price;
        lottery.end_time_stamp = end_time_stamp;
        Ok(())
    }

    pub fn buy_ticket(ctx : Context<BuyTicket>) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        let buyer = &mut ctx.accounts.buyer;
        invoke(
            &system_instruction::transfer(&buyer.key(), &lottery.key(), lottery.price),
            &[
                buyer.to_account_info(),
                lottery.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        lottery.participants.push(buyer.key());
        lottery.sold += 1;
        Ok(())
    }

    pub fn determin_winner(ctx : Context<DeterminWinner>) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        let clock = Clock::get()?;
        let random_seed = clock.unix_timestamp;
        let winner_index = (random_seed as usize) % lottery.participants.len();
        lottery.won = Some(lottery.participants[winner_index]);
        Ok(())
    }

    pub fn claim_reward(ctx : Context<Reward>) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        let winner = &ctx.accounts.winner;

        require!(lottery.won.is_some(), LotteryError::NoWinnerYet);
        require!(lottery.won.unwrap() == *winner.key, LotteryError::InvalidWinner);

        let balance = &lottery.to_account_info().lamports();

        invoke(&system_instruction::transfer(&lottery.key(), &winner.key(), *balance), 
            &[
            lottery.to_account_info(),
            winner.to_account_info(),
            ctx.accounts.system_program.to_account_info()
        ])?;
        Ok(())
    }
}


#[derive(Accounts)]
#[instruction(price: u64, end_time_stamp: i64)]
pub struct Initialize<'info>{
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 4 + (4 + 32 * 100) + 4 + 8 + 8 + 32
    )]
    pub lottery : Account<'info , Lottery>,
    #[account(mut)]
    pub authority : Signer<'info>,
    pub system_program : Program<'info , System>
}

#[derive(Accounts)]
pub struct BuyTicket<'info>{
    #[account(mut, seeds = [b"lottery"], bump)]
    pub lottery : Account<'info , Lottery>,
    #[account(mut)]
    pub buyer : Signer<'info>,
    pub system_program : Program<'info , System>
}

#[derive(Accounts)]
pub struct DeterminWinner<'info>{
    #[account(mut, seeds = [b"lottery"], bump)]
    pub lottery : Account<'info , Lottery>,
}

#[derive(Accounts)]
pub struct Reward<'info>{
    #[account(mut, seeds = [b"lottery"], bump)]
    pub lottery : Account<'info , Lottery>,
    #[account(mut)]
    pub winner : Signer<'info>,
    pub system_program : Program<'info , System>
}


#[account]
pub struct Lottery{
    pub authority : Pubkey,
    pub participants : Vec<Pubkey>,
    pub price : u64,
    pub end_time_stamp : i64,
    pub sold : u64,
    pub won : Option<Pubkey>
}


#[error_code]
pub enum LotteryError {
    #[msg("Lottery has not ended yet")]
    LotteryNotEnded,
    #[msg("No winner determined yet")]
    NoWinnerYet,
    #[msg("Invalid winner")]
    InvalidWinner,
}
