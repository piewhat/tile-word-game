use crate::{
    components::{placed::Placed, slot::Slot, tile::Tile},
    events::MoveTile,
};
use bevy::prelude::*;

pub fn handle_tile_movement(
    mut commands: Commands,
    mut move_events: EventReader<MoveTile>,
    mut tiles: Query<&mut Transform, With<Tile>>,
    mut slots: Query<(Entity, &mut Slot, &Transform), Without<Tile>>,
) {
    for MoveTile { tile, target_slot } in move_events.read() {
        // Get the tile's current transform and position
        let tile_transform = {
            let Ok(tile_transform) = tiles.get(*tile) else {
                continue;
            };
            tile_transform.clone()
        };

        if let Some(slot_entity) = target_slot {
            // Moving to a slot
            let Ok((_, mut slot, slot_transform)) = slots.get_mut(*slot_entity) else {
                continue;
            };

            // If slot has occupant, swap them
            if let Some(occupant) = slot.occupant {
                if let Ok(mut occupant_transform) = tiles.get_mut(occupant) {
                    occupant_transform.translation = tile_transform.translation;
                    commands.entity(occupant).remove::<Placed>();
                }
            }

            // Move tile to slot
            let mut tile_transform = tiles.get_mut(*tile).unwrap();
            tile_transform.translation = slot_transform.translation;
            slot.occupant = Some(*tile);

            // Mark as placed only if not a grid slot
            if slot.is_grid {
                commands.entity(*tile).remove::<Placed>();
            } else {
                commands.entity(*tile).insert(Placed);
            }
        } else {
            // Moving to free space - just remove Placed component
            commands.entity(*tile).remove::<Placed>();
        }
    }
}
