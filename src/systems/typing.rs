use crate::components::{
    grid::Grid,
    slot::{Slot, SwapSlot},
    tile::Tile,
};
use crate::events::SubmitWord;
use bevy::prelude::*;

pub fn handle_typing(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    grid_slots: Query<(Entity, &Slot), (With<Slot>, With<Grid>)>,
    available_slots: Query<(Entity, &Slot, &Transform), (With<Slot>, Without<Grid>)>,
    tiles: Query<&Text2d, With<Tile>>,
) {
    // Check for letter key presses
    for key in keyboard_input.get_just_pressed() {
        if let Some(letter) = keycode_to_char(*key) {
            // Find empty available slots
            let mut available_slots_vec: Vec<_> =
                available_slots.iter().filter(|s| s.1.0.is_none()).collect();

            // Sort by x coordinate to find leftmost slot
            available_slots_vec
                .sort_by(|a, b| a.2.translation.x.partial_cmp(&b.2.translation.x).unwrap());

            if let Some((available_slot_entity, _, _)) = available_slots_vec.first() {
                // Find a grid slot with a tile matching the letter
                if let Some((matching_tile_slot_entity, _)) = grid_slots.iter().find(|(_, slot)| {
                    if let Some(slot) = slot.0
                        && let Ok(text) = tiles.get(slot)
                        && text.0 == letter.to_string()
                    {
                        true
                    } else {
                        false
                    }
                }) {
                    // Mark available slot for swap with matching grid slot
                    commands
                        .entity(matching_tile_slot_entity)
                        .insert(SwapSlot(*available_slot_entity));
                }
            }
        } else if key == &KeyCode::Backspace {
            let mut occupied_slots_vec: Vec<_> =
                available_slots.iter().filter(|s| s.1.0.is_some()).collect();

            occupied_slots_vec
                .sort_by(|a, b| b.2.translation.x.partial_cmp(&a.2.translation.x).unwrap());

            if let Some((occupied_slot_entity, _, _)) = occupied_slots_vec.first() {
                if let Some((available_grid_slot_entity, _)) =
                    grid_slots.iter().find(|(_, slot)| slot.0.is_none())
                {
                    commands
                        .entity(*occupied_slot_entity)
                        .insert(SwapSlot(available_grid_slot_entity));
                }
            }
        } else if key == &KeyCode::Enter {
            let mut occupied_slots_vec: Vec<_> =
                available_slots.iter().filter(|s| s.1.0.is_some()).collect();

            occupied_slots_vec
                .sort_by(|a, b| a.2.translation.x.partial_cmp(&b.2.translation.x).unwrap());

            let mut word = String::new();
            for (_, slot, _) in occupied_slots_vec {
                if let Some(tile_entity) = slot.0 {
                    if let Ok(text) = tiles.get(tile_entity) {
                        word.push_str(&text.0);
                    }
                }
            }

            if !word.is_empty() {
                commands.trigger(SubmitWord { word });
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
