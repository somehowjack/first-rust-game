use std::{ops::Range, ops::RangeFrom};
use crate::status::Status;
use crate::entity::Entity;
use crate::target::Target;


pub enum Effect_Amount {
    Range(Range<usize>),
    Number(usize)
}

pub enum Effect{ 
    Reg_Status{
        target: Target,
        status: Option<Status>,
        to_hit: usize // out of 100 (greater than 100 is sure to hit)
    },
    Reg_Heal{
        target: Target,
        heal_amount: Effect_Amount,
        status: Option<Status>,
        to_hit: usize // out of 100 (greater than 100 is sure to hit)
    },
    Reg_Attack{
        target: Target,
        dmg_amount: Effect_Amount,
        status: Option<Status>,
        to_hit: usize // out of 100 (greater than 100 is sure to hit)
    },
    Reg_Leech{ // heals one target on hit of another
        att_target: Target,
        heal_target: Target,
        dmg_amount: Effect_Amount,
        status: Option<Status>,
        to_hit: usize // out of 100 (greater than 100 is sure to hit)
    },
    // Mult effects apply a number of times equal to the times_repeated
    Mult_Status{
        target: Target,
        times_repeated: Effect_Amount,
        status: Option<Status>,
        to_hit: usize // out of 100 (greater than 100 is sure to hit)
    },
    Mult_Heal{
        target: Target,
        heal_amount: Effect_Amount,
        times_repeated: Effect_Amount,
        status: Option<Status>,
        to_hit: usize // out of 100 (greater than 100 is sure to hit)
    },
    Mult_Attack{
        target: Target,
        dmg_amount: Effect_Amount,
        times_repeated: Effect_Amount,
        status: Option<Status>,
        to_hit: usize // out of 100 (greater than 100 is sure to hit)
    }
}

