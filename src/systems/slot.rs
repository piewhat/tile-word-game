use std::mem;

use bevy::prelude::*;

use crate::components::{
    slot::{Slot, SlotUpdate, SwapSlot},
    tile::Tile,
};

pub fn swap_slots(
    mut commands: Commands,
    mut swap_slots: Query<
        (&mut Slot, &mut SwapSlot, &Transform, Entity),
        (With<Slot>, Without<Tile>),
    >,
    mut slots: Query<(&mut Slot, &Transform), (With<Slot>, Without<Tile>, Without<SwapSlot>)>,
    mut tiles: Query<&mut Transform, (With<Tile>, Without<Slot>)>,
) {
    for (mut slot1, swap_slot, slot1_transform, swap_slot_entity) in swap_slots.iter_mut() {
        if let Ok((mut slot2, slot2_transform)) = slots.get_mut(swap_slot.0)
            && let Some(tile1) = slot1.0
        {
            if let Some(tile2) = slot2.0 {
                let [mut tile1, mut tile2] = tiles.get_many_mut([tile1, tile2]).unwrap();
                tile1.translation = slot2_transform.translation;
                tile2.translation = slot1_transform.translation;
            } else {
                let mut tile1 = tiles.get_mut(tile1).unwrap();
                tile1.translation = slot2_transform.translation;
            }
            mem::swap(&mut slot1.0, &mut slot2.0);
        }

        commands.entity(swap_slot_entity).remove::<SwapSlot>();
    }
}

pub fn update_slots(
    mut commands: Commands,
    update_slots: Query<(&Slot, Entity, &Transform), (With<Slot>, With<SlotUpdate>)>,
    mut tiles: Query<&mut Transform, (With<Tile>, Without<Slot>)>,
) {
    for (slot, slot_entity, slot_transform) in update_slots.iter() {
        if let Some(tile) = slot.0 {
            let mut tile = tiles.get_mut(tile).unwrap();
            tile.translation = slot_transform.translation;
            commands.entity(slot_entity).remove::<SlotUpdate>();
        }
    }
}
