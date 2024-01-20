use crate::entity::Entity;



pub enum Target { // representation of what an effect targets
    Target_All,
    Target_Self,
    Target_Enemy,
    Target_Enemy_Ajacent, // attacks target and enemies next to target
    Target_Team,
    Whole_Enemy,
    Whole_Team,
    Rand_Enemy, // mult effects pick a random enemy EACH TIME
    Rand_Team, // mult effects pick a random team EACH TIME
    Rand_Enemy_Concrete, // mult effects apply to same random enemy for the whole effect
    Rand_Team_Concrete, // mult effects apply to same random team for the whole effect
}

impl Target {
    /**
     * Returns Entity based on Target type and Originator's team
     */
    fn get_target(&self, /*game: &Game,*/ originator: Entity) -> Entity { // Returns entity for effect to be applied to 
        
        match self {
            Target::Target_All => todo!(),
            Target::Target_Self => todo!(),
            Target::Target_Enemy => todo!(),
            Target::Target_Enemy_Ajacent => todo!(),
            Target::Target_Team => todo!(),
            Target::Whole_Enemy => todo!(),
            Target::Whole_Team => todo!(),
            Target::Rand_Enemy => todo!(),
            Target::Rand_Team => todo!(),
            Target::Rand_Enemy_Concrete => todo!(),
            Target::Rand_Team_Concrete => todo!(),
        }
    }
}