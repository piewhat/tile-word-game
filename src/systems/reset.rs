use bevy::prelude::*;

use crate::components::{grid::Grid, occupied::Occupied, placed::Placed, slot::Slot, tile::Tile};

pub fn check_spacebar_and_reset(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut placed_tiles: Query<(Entity, &mut Transform), (With<Tile>, With<Placed>)>,
    board_tiles: Query<&mut Transform, (With<Tile>, Without<Placed>)>,
    grid_slots: Query<&Transform, (With<Slot>, With<Grid>, Without<Tile>)>,
    occupied_slots: Query<Entity, (With<Slot>, With<Occupied>)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let mut placed_tiles = placed_tiles.iter_mut();
        for slot_transform in grid_slots.iter() {
            let slot_position = slot_transform.translation.truncate();

            // Check if there's already a tile at this slot position
            let tile_at_slot = board_tiles
                .iter()
                .any(|tile_transform| tile_transform.translation.truncate() == slot_position);

            // If no tile is at this slot position, move a reset tile here
            if !tile_at_slot {
                if let Some((tile_entity, mut tile_transform)) = placed_tiles.next() {
                    // Remove Placed component from the tile
                    commands.entity(tile_entity).remove::<Placed>();
                    tile_transform.translation = slot_transform.translation;
                }
            }
        }

        for slot_entity in occupied_slots.iter() {
            commands.entity(slot_entity).remove::<Occupied>();
        }
    }
}
