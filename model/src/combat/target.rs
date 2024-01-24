use crate::entities::entity::Entity;



pub enum Target { // representation of what an effect targets
    TargetAll,
    TargetSelf,
    TargetEnemy,
    TargetEnemyAjacent, // attacks target and enemies next to target
    TargetTeam,
    WholeEnemy,
    WholeTeam,
    RandEnemy, // mult effects pick a random enemy EACH TIME
    RandTeam, // mult effects pick a random team EACH TIME
    RandEnemyConcrete, // mult effects apply to same random enemy for the whole effect
    RandTeamConcrete, // mult effects apply to same random team for the whole effect
}

impl Target {
    /**
     * Returns Entity based on Target type and Originator's team
     */
    fn get_target(&self, /*game: &Game,*/ originator: Entity) -> Entity { // Returns entity for effect to be applied to 
        
        match self {
            Target::TargetAll => todo!(),
            Target::TargetSelf => todo!(),
            Target::TargetEnemy => todo!(),
            Target::TargetEnemyAjacent => todo!(),
            Target::TargetTeam => todo!(),
            Target::WholeEnemy => todo!(),
            Target::WholeTeam => todo!(),
            Target::RandEnemy => todo!(),
            Target::RandTeam => todo!(),
            Target::RandEnemyConcrete => todo!(),
            Target::RandTeamConcrete => todo!(),
        }
    }
}