
pub type status_stacks = u32;

pub enum Status {
    Burn(status_stacks),
    Poison(status_stacks),
    Regen(status_stacks)
}