use crate::{combat::action::Action, entities::stats::EntityModifier};

pub enum Equipment_Slot {
    Head,
    Chest,
    Legs,
    Gloves,
    Boots,
    Ring,
    OneHand,
    TwoHand,
    Background
}

pub enum Item {
    Equipment{
        slot: Equipment_Slot,
        actions: Vec<Action>,
        entity_modifiers: Vec<EntityModifier>
    },
    Spell{
        actions: Vec<Action>,
        mana_cost: usize
    }
}