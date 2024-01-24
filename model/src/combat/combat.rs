use crate::entities::status::Status;
use crate::entities::entity::Entity;
use super::target::Target;


pub struct Combat {
    team: Vec<Entity>,
    enemy: Vec<Entity>,
}