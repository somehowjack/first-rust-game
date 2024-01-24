use crate::entities::status::Status;
use crate::entities::entity::Entity;
use super::target::Target;


enum CombatPhase{
    Start,
    TeamTurn(Entity),
    EnemyTurn(Entity),
    End
}

pub struct Combat {
    team: Vec<Entity>,
    enemy: Vec<Entity>,
    phases: Vec<CombatPhase>
}