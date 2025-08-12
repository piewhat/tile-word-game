use bevy::ecs::{component::Component, entity::Entity};

#[derive(Component)]
pub struct PlaceTile(pub Option<Entity>);
