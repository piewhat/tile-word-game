use bevy::prelude::*;

use crate::components::{
    grid::Grid,
    slot::{Slot, SwapSlot},
};

pub fn check_spacebar_and_reset(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    top_slots: Query<(&Slot, Entity), Without<Grid>>,
    grid_slots: Query<(&Slot, Entity), With<Grid>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let mut slots_to_reset = vec![];
        for (slot, entity) in top_slots {
            if slot.0.is_some() {
                slots_to_reset.push(entity);
            }
        }

        for (slot, entity) in grid_slots {
            if slot.0.is_none() {
                commands
                    .entity(slots_to_reset.pop().unwrap())
                    .insert(SwapSlot(entity));
            }
        }
    }
}
