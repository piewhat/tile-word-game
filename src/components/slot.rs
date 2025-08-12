use bevy::ecs::component::Component;

#[derive(Component, Default)]
pub struct Slot(pub Option<i32>);
