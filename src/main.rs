use bevy::{color::palettes::css::*, prelude::*, text::TextBounds};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use fastrand;

mod components;
mod systems;

use crate::{
    components::{
        grid::Grid,
        slot::{Slot, SlotBundle, SlotUpdate, SwapSlot},
        tile::{Tile, TileBundle, TileScoreBundle},
    },
    systems::{
        cursor::cursor,
        reset::check_spacebar_and_reset,
        slot::{swap_slots, update_slots},
        typing::handle_typing,
    },
};

#[derive(Resource, Default)]
struct WorldCoords(Vec2);

#[derive(Resource, Default)]
struct DragOrigin(Option<Vec3>);

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::srgb(1., 1., 1.)))
        .init_resource::<WorldCoords>()
        .init_resource::<DragOrigin>()
        .add_systems(Startup, add_tile)
        .add_systems(
            Update,
            (
                cursor,
                check_spacebar_and_reset,
                handle_typing,
                swap_slots,
                update_slots,
            ),
        )
        .run();
}

fn add_tile(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, MainCamera));

    for r in -2..2 {
        for c in -2..2 {
            let random_char = (b'A' + fastrand::u8(0..26) as u8) as char;
            let tile = commands
                .spawn(TileBundle::new(
                    random_char,
                    asset_server.load("fonts/Karla-ExtraBold.ttf"),
                    asset_server.load("box.png"),
                    Transform::from_xyz((r as f32 + 0.5) * 110., (c as f32 - 1.) * 110., 0.),
                ))
                .with_child(TileScoreBundle::new(
                    '1',
                    asset_server.load("fonts/Roboto-Regular.ttf"),
                ))
                .observe(update_tile_position)
                .observe(tile_startdrag)
                .id();

            commands
                .spawn((
                    SlotBundle::new(
                        Some(tile),
                        asset_server.load("fonts/Roboto-Regular.ttf"),
                        asset_server.load("box.png"),
                        Transform::from_xyz((r as f32 + 0.5) * 110., (c as f32 - 1.) * 110., 0.),
                    ),
                    Grid,
                ))
                .observe(slot_to_slot)
                .observe(slot_enddrag);
        }
    }

    for i in -5..5 {
        commands
            .spawn((
                SlotBundle::new(
                    None,
                    asset_server.load("fonts/Roboto-Regular.ttf"),
                    asset_server.load("box.png"),
                    Transform::from_xyz((i as f32 + 0.5) * 110., 200., 0.),
                ),
                Text2d::new(format!("\n\n\n{}", i + 6)),
            ))
            .observe(slot_to_slot)
            .observe(slot_enddrag);
    }

    commands
        .spawn((
            Sprite::from_image(asset_server.load("shuffle.png")),
            Transform::from_xyz(-275., -330., 0.),
            Pickable {
                should_block_lower: true,
                is_hoverable: true,
            },
        ))
        .with_child((
            Sprite::from_image(asset_server.load("box.png")),
            Transform::from_xyz(0., 0., -1.),
            Pickable {
                should_block_lower: false,
                is_hoverable: true,
            },
        ))
        .observe(shuffle);
}

fn shuffle(
    _click: On<Pointer<Click>>,
    mut commands: Commands,
    mut slots: Query<(&mut Slot, Entity, &Transform), With<Grid>>,
) {
    let mut tiles = slots.iter().filter_map(|f| f.0.0).collect::<Vec<_>>();
    let mut all_slots: Vec<_> = slots.iter_mut().collect();
    all_slots.sort_by(|a, b| {
        a.2.translation
            .y
            .partial_cmp(&b.2.translation.y)
            .unwrap()
            .then(b.2.translation.x.partial_cmp(&a.2.translation.x).unwrap())
    });

    fastrand::shuffle(&mut tiles);
    all_slots.reverse();

    for (mut slot, slot_entity, _) in all_slots {
        if let Some(tile_entity) = tiles.pop() {
            slot.0 = Some(tile_entity);
            commands.entity(slot_entity).insert(SlotUpdate);
        } else {
            slot.0 = None
        }
    }
}

fn slot_to_slot(
    drag: On<Pointer<DragDrop>>,
    mut commands: Commands,
    slots: Query<&Slot>,
    mut drag_origin: ResMut<DragOrigin>,
) {
    if slots.contains(drag.dropped) && slots.contains(drag.entity) && drag.dropped != drag.entity {
        drag_origin.0 = None;
        commands.entity(drag.dropped).insert(SwapSlot(drag.entity));
    }
}

fn tile_startdrag(
    drag: On<Pointer<DragStart>>,
    mut drag_origin: ResMut<DragOrigin>,
    transforms: Query<&Transform>,
) {
    if let Ok(transform) = transforms.get(drag.entity) {
        drag_origin.0 = Some(transform.translation);
    }
}

fn slot_enddrag(
    drag: On<Pointer<DragEnd>>,
    mut drag_origin: ResMut<DragOrigin>,
    slots: Query<&Slot>,
    mut transforms: Query<&mut Transform, With<Tile>>,
) {
    if let Some(origin) = drag_origin.0 {
        if let Ok(slot) = slots.get(drag.entity) {
            if let Some(slot) = slot.0 {
                if let Ok(mut transform) = transforms.get_mut(slot) {
                    transform.translation = origin;
                    drag_origin.0 = None;
                }
            }
        }
    }
}

fn update_tile_position(drag: On<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    if let Ok(mut transform) = transforms.get_mut(drag.entity) {
        transform.translation.x += drag.delta.x;
        transform.translation.y += -drag.delta.y;
        transform.translation.z = 1.;
    }
}
