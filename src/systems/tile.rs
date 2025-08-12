use bevy::prelude::*;

use crate::components::{
    grid::Grid, occupied::Occupied, place_tile::PlaceTile, placed::Placed, slot::Slot, tile::Tile,
};

pub fn place_tile(
    mut commands: Commands,
    mut tiles_to_place: Query<(Entity, &mut Transform, &PlaceTile)>,
    mut slots: Query<(&mut Transform, Option<&Occupied>, Has<Grid>), (With<Slot>, Without<Tile>)>,
    mut tiles: Query<&mut Transform, (With<Tile>, Without<Slot>)>,
) {
    for (tile_entity, mut tile_transform, place_tile) in tiles_to_place {
        if let Some(slot_entity) = place_tile.0 {
            let (slot_transform, occupied, has_grid) = slots.get_mut(slot_entity).unwrap();

            if let Some(occupied) = occupied
                && let Some(occupied_tile_entity) = occupied.0
            {
                let mut occupied_tile_transform = tiles.get_mut(occupied_tile_entity).unwrap();
                occupied_tile_transform.translation = tile_transform.translation;
                commands.entity(occupied_tile_entity).remove::<Placed>();
            }

            tile_transform.translation = slot_transform.translation;

            if has_grid {
                commands.entity(tile_entity).remove::<Placed>();
            } else {
                commands
                    .entity(slot_entity)
                    .insert(Occupied(Some(tile_entity)));
                commands.entity(tile_entity).insert(Placed);
            }
        }
    }
}
