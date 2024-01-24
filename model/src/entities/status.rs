use super::stats::EntityModifier;

pub type status_stacks = usize;

pub enum Status {
    Burn(status_stacks),
    Poison(status_stacks),
    Regen(status_stacks),
    Modify(EntityModifier, status_stacks)
}