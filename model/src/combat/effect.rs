use std::ops::Range;
use super::combat::Combat;
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
        status: Status,
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
        status: Status,
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

impl Effect {
    /**
     * Executes all effects on the 
     */
    pub fn exec_effect(&self, combat: &Combat, originator: &Entity) { 
        let mut effect_target: Entity;
        
        match self {
            Effect::RegStatus { target, status, to_hit } => {
                effect_target = target.get_target(combat, originator);
            },
            Effect::RegHeal { target, heal_amount, status, to_hit } => todo!(),
            Effect::RegAttack { target, dmg_amount, status, to_hit } => todo!(),
            Effect::RegLeech { att_target, heal_target, dmg_amount, status, to_hit } => todo!(),
            Effect::MultStatus { target, times_repeated, status, to_hit } => todo!(),
            Effect::MultHeal { target, heal_amount, times_repeated, status, to_hit } => todo!(),
            Effect::MultAttack { target, dmg_amount, times_repeated, status, to_hit } => todo!(),
            Effect::MultLeech { att_target, heal_target, dmg_amount, times_repeated, status, to_hit } => todo!(),
        }
        todo!()
    }

    pub(super) fn get_number_of_targets(&self) -> i32 {
        let out = 0;

        match self {
            Effect::RegLeech { att_target, heal_target, .. } | Effect::MultLeech { att_target, heal_target, .. } => {
                todo!()
            },
            _ => todo!()
            
        }
        out
    }
}

