use std::ops::Range;
use super::target::Target;
use crate::entities::status::Status;
use crate::entities::entity::Entity;


pub enum EffectAmount {
    Range(Range<usize>),
    Number(usize)
}

pub enum Effect{ 
    RegStatus{
        target: Target,
        status: Option<Status>,
        to_hit: usize // out of 100 (greater than 100 is sure to hit)
    },
    RegHeal{
        target: Target,
        heal_amount: EffectAmount,
        status: Option<Status>,
        to_hit: usize // out of 100 (greater than 100 is sure to hit)
    },
    RegAttack{
        target: Target,
        dmg_amount: EffectAmount,
        status: Option<Status>,
        to_hit: usize // out of 100 (greater than 100 is sure to hit)
    },
    RegLeech{ // heals one target on hit of another
        att_target: Target,
        heal_target: Target,
        dmg_amount: EffectAmount,
        status: Option<Status>,
        to_hit: usize // out of 100 (greater than 100 is sure to hit)
    },
    // Mult effects apply a number of times equal to the times_repeated
    MultStatus{
        target: Target,
        times_repeated: EffectAmount,
        status: Option<Status>,
        to_hit: usize // out of 100 (greater than 100 is sure to hit)
    },
    MultHeal{
        target: Target,
        heal_amount: EffectAmount,
        times_repeated: EffectAmount,
        status: Option<Status>,
        to_hit: usize // out of 100 (greater than 100 is sure to hit)
    },
    MultAttack{
        target: Target,
        dmg_amount: EffectAmount,
        times_repeated: EffectAmount,
        status: Option<Status>,
        to_hit: usize // out of 100 (greater than 100 is sure to hit)
    },
    MultLeech{ // heals one target on hit of another
        att_target: Target,
        heal_target: Target,
        dmg_amount: EffectAmount,
        times_repeated: EffectAmount,
        status: Option<Status>,
        to_hit: usize // out of 100 (greater than 100 is sure to hit)
    },
}

