use crate::{combat::action::Action, entities::stats::StatModifier};

pub enum Equipment_Slot {
    Head,
    Chest,
    Legs,
    Gloves,
    Boots,
    Ring,
    OneHand,
    TwoHand
}

pub enum Item {
    Equipment{
        slot: Equipment_Slot,
        actions: Vec<Action>,
        stat_modifiers: Vec<StatModifier>
    },
    Spell{
        actions: Vec<Action>,
        mana_cost: usize
    }
}