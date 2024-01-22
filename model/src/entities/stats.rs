/*

NOT DONE: Make a method that returns a description of the stat/attr.

NOT DONE: Make a string mthd for StatModifier/ AttrModifier

*/

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

pub enum StatModifier {
    Str(isize),
    Dex(isize),
    Con(isize),
    Cha(isize),
    Int(isize),
    Wis(isize),
    All(isize)
}

pub enum AttrModifier {
    PhyDef(isize),
    MagDef(isize)
}

