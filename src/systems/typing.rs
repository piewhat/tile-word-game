use crate::components::{grid::Grid, occupied::Occupied, placed::Placed, slot::Slot, tile::Tile};
use bevy::prelude::*;

pub fn handle_typing(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut available_tiles: Query<
        (Entity, &mut Transform, &Text2d),
        (With<Tile>, Without<Placed>, Without<Slot>),
    >,
    available_slots: Query<(Entity, &Transform), (With<Slot>, Without<Grid>, Without<Occupied>)>,
) {
    // Check for letter key presses
    for key in keyboard_input.get_just_pressed() {
        if let Some(letter) = keycode_to_char(*key) {
            // Find the leftmost available slot
            let available_slot = find_leftmost_available_slot(&available_slots);

            if let Some((slot_entity, slot_transform)) = available_slot {
                // Find and move a matching tile
                if let Some((tile_entity, mut tile_transform, _)) = available_tiles
                    .iter_mut()
                    .find(|(_, _, text)| text.0.chars().next() == Some(letter))
                {
                    // Move tile to new slot
                    tile_transform.translation = slot_transform.translation;
                    tile_transform.translation.z = 0.0;

                    // Mark the tile as placed and the slot as occupied
                    commands.entity(tile_entity).insert(Placed);
                    commands.entity(slot_entity).insert(Occupied);
                }
            }
        }
    }
}

fn keycode_to_char(keycode: KeyCode) -> Option<char> {
    match keycode {
        KeyCode::KeyA => Some('A'),
        KeyCode::KeyB => Some('B'),
        KeyCode::KeyC => Some('C'),
        KeyCode::KeyD => Some('D'),
        KeyCode::KeyE => Some('E'),
        KeyCode::KeyF => Some('F'),
        KeyCode::KeyG => Some('G'),
        KeyCode::KeyH => Some('H'),
        KeyCode::KeyI => Some('I'),
        KeyCode::KeyJ => Some('J'),
        KeyCode::KeyK => Some('K'),
        KeyCode::KeyL => Some('L'),
        KeyCode::KeyM => Some('M'),
        KeyCode::KeyN => Some('N'),
        KeyCode::KeyO => Some('O'),
        KeyCode::KeyP => Some('P'),
        KeyCode::KeyQ => Some('Q'),
        KeyCode::KeyR => Some('R'),
        KeyCode::KeyS => Some('S'),
        KeyCode::KeyT => Some('T'),
        KeyCode::KeyU => Some('U'),
        KeyCode::KeyV => Some('V'),
        KeyCode::KeyW => Some('W'),
        KeyCode::KeyX => Some('X'),
        KeyCode::KeyY => Some('Y'),
        KeyCode::KeyZ => Some('Z'),
        _ => None,
    }
}

fn find_leftmost_available_slot(
    available_slots: &Query<(Entity, &Transform), (With<Slot>, Without<Grid>, Without<Occupied>)>,
) -> Option<(Entity, Transform)> {
    let mut slots: Vec<_> = available_slots.iter().collect();

    // Sort by x coordinate to find leftmost slot
    slots.sort_by(|a, b| a.1.translation.x.partial_cmp(&b.1.translation.x).unwrap());

    slots
        .first()
        .map(|(entity, transform)| (*entity, **transform))
}
