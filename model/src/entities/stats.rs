use crate::combat::action::Type;

pub enum Stat {
    Str,
    Dex,
    Con,
    Cha,
    Int,
    Wis
}

pub enum Attr {
    PhyDef,
    MagDef
}

pub enum EntityModifier {
    StatModifier(Stat, i32),
    AttrModifier(Attr, i32),
    TypeResistance(Type),
    TypeImmunity(Type)
}

