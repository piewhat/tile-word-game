use bevy::ecs::{component::Component, entity::Entity};

#[derive(Component, Default)]
pub struct Occupied(pub Option<Entity>);
