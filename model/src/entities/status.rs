use super::stats::StatModifier;

pub type status_stacks = usize;

pub enum Status {
    Burn(status_stacks),
    Poison(status_stacks),
    Regen(status_stacks),
    Stat(StatModifier, status_stacks)
}