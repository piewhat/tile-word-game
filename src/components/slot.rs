use bevy::ecs::{component::Component, entity::Entity};

#[derive(Component, Default)]
pub struct Slot(pub Option<Entity>);

#[derive(Component)]
pub struct SwapSlot(pub Entity);

#[derive(Component)]
pub struct SlotUpdate;
