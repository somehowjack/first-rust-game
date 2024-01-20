use crate::entity::Entity;
use crate::status::{Status, status_stacks};

pub enum Effect{
    ApplyStatus(
        Entity,
        Status
    ),
    ApplyHeal,
    ApplyAttack
}