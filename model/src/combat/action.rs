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

pub enum ActionType {
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
    action_types: Vec<ActionType>
}

impl Action {
    /**
     * Returns Entity based on Target type and Originator's team
     */
    fn exec_action(&self, combat: &Combat, originator: &Entity, target: &Entity) { // Returns entity for effect to be applied to 
        

        todo!()
    }
}