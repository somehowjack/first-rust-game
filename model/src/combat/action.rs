/*

TODO: 

DONE: Make action struct, actions should have an action type and a collection of effects

NOT DONE: Make a method that takes in the game object, the originator (entity), the target (option entity) and executes all effects

NOT DONE: Get description mthd

NOT DONE: Make a debug string mthd

NOT DONE: Make a string mthd

*/

use crate::entities::entity::Entity;
use super::effect::Effect;
use super::combat::Combat;

pub enum Type {
    Physical,
    Radiant,
    Necrotic,
    Fire,
    Cold,
    Lightning,
    Acid,
    // more in the future
}

pub struct Action {
    name: String,
    desc: String,
    effects: Vec<Effect>,
    action_types: Vec<Type>
}

impl Action {
    /**
     * Executes all effects in order.
     */
    fn exec_action(&self, combat: &Combat, originator: &Entity, target: Option<&Entity>) { 
        for effect in &self.effects {
            effect.exec_effect(combat, originator);
        }
        todo!()
    }

    /**
     * Number of targets needed for the action
     */
    fn get_action_number_of_targets(&self) -> i32{
        let output = 0;

        for effect in &self.effects {
            effect.get_number_of_targets();
        }
        0
    }
}