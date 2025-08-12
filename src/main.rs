mod components;
mod systems;

use bevy::{color::palettes::css::*, prelude::*, text::TextBounds};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use fastrand;

use crate::{
    components::{
        dragging::Dragging, grid::Grid, occupied::Occupied, place_tile::PlaceTile, placed::Placed,
        slot::Slot, tile::Tile,
    },
    systems::{cursor::cursor, reset::check_spacebar_and_reset, typing::handle_typing},
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
        .add_systems(Update, (cursor, check_spacebar_and_reset, handle_typing))
        .run();
}

fn add_tile(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, MainCamera));

    for i in -5..5 {
        commands.spawn((
            Slot(Some(i + 5)),
            Text2d::new(format!("\n\n\n{}", i + 6)),
            TextFont {
                font: asset_server.load("fonts/Roboto-Regular.ttf"),
                font_size: 20.0,
                ..Default::default()
            },
            TextColor(BLACK.into()),
            TextLayout::new_with_justify(JustifyText::Right),
            TextBounds::from(Vec2::new(89., 95.)),
            Sprite::from_image(asset_server.load("box.png")),
            Transform::from_xyz((i as f32 + 0.5) * 110., 200., 0.),
        ));
    }

    for r in -2..2 {
        for c in -2..2 {
            commands.spawn((
                Slot::default(),
                Grid,
                TextFont {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 20.0,
                    ..Default::default()
                },
                TextColor(BLACK.into()),
                TextLayout::new_with_justify(JustifyText::Right),
                TextBounds::from(Vec2::new(89., 95.)),
                Sprite::from_image(asset_server.load("box.png")),
                Transform::from_xyz((r as f32 + 0.5) * 110., (c as f32 - 1.) * 110., 0.),
            ));
        }
    }

    for r in -2..2 {
        for c in -2..2 {
            let random_char = (b'A' + fastrand::u8(0..26) as u8) as char;
            commands
                .spawn((
                    Tile,
                    Text2d::new(random_char),
                    TextFont {
                        font: asset_server.load("fonts/Karla-ExtraBold.ttf"),
                        font_size: 75.0,
                        ..Default::default()
                    },
                    TextColor(BLACK.into()),
                    Sprite::from_image(asset_server.load("box.png")),
                    Transform::from_xyz((r as f32 + 0.5) * 110., (c as f32 - 1.) * 110., 0.),
                    Pickable::default(),
                ))
                .with_child((
                    Text2d::new('1'),
                    TextFont {
                        font: asset_server.load("fonts/Roboto-Regular.ttf"),
                        font_size: 20.0,
                        ..Default::default()
                    },
                    TextColor(BLACK.into()),
                    TextLayout::new_with_justify(JustifyText::Right),
                    TextBounds::from(Vec2::new(89., 95.)),
                ))
                .observe(update_tile_position)
                .observe(handle_tile_drop)
                .observe(start_tile_drag)
                .observe(handle_tile_grab);
        }
    }
}

fn start_tile_drag(
    drag: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    mut transforms: Query<&mut Transform>,
    mut drag_origin: ResMut<DragOrigin>,
    world_coords: Res<WorldCoords>,
) {
    if let Ok(mut transform) = transforms.get_mut(drag.target()) {
        let mouse_world_pos = world_coords.0.extend(0.);
        drag_origin.0 = Some(transform.translation);
        transform.translation = mouse_world_pos;
        commands.entity(drag.target()).insert(Dragging);
    }
}

fn update_tile_position(drag: Trigger<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    if let Ok(mut transform) = transforms.get_mut(drag.target()) {
        transform.translation.x += drag.delta.x;
        transform.translation.y += -drag.delta.y;
        transform.translation.z = 1.;
    }
}

fn handle_tile_grab(
    _drag: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    mut occupied_slots: Query<(Entity, &Transform), (With<Slot>, With<Occupied>)>,
    world_coords: Res<WorldCoords>,
) {
    let mouse_pos = world_coords.0.extend(0.);
    for (entity, transform) in occupied_slots.iter_mut() {
        if transform.translation.distance(mouse_pos) < 50. {
            commands.entity(entity).remove::<Occupied>(); // this is broken if a tile is dropped over empty space maybe make a standerized way to add a tile to a slot something like PlaceTile(Option<Entity>)
            break;
        }
    }
}

fn handle_tile_drop(
    drag: Trigger<Pointer<DragEnd>>,
    mut commands: Commands,
    mut drag_tile: Query<&mut Transform, With<Dragging>>,
    mut tiles_and_slots: Query<
        (Entity, &mut Transform, Has<Tile>, Has<Slot>, Has<Grid>),
        Without<Dragging>,
    >,
    mut drag_origin: ResMut<DragOrigin>,
    world_coords: Res<WorldCoords>,
) {
    let Ok(mut d_tile_transform) = drag_tile.single_mut() else {
        commands.entity(drag.target()).remove::<Dragging>();
        return;
    };

    let Some(origin) = drag_origin.0 else {
        commands.entity(drag.target()).remove::<Dragging>();
        drag_origin.0 = None;
        return;
    };

    let mouse_pos = world_coords.0.extend(0.);

    let mut reset = true;
    let mut tile_processed = false;
    let mut slot_processed = false;

    for (entity, mut transform, has_tile, has_slot, has_grid) in tiles_and_slots.iter_mut() {
        if transform.translation.distance(mouse_pos) < 50. {
            if has_tile && !tile_processed {
                transform.translation = origin;
                tile_processed = true;
            }
            if has_slot && !slot_processed {
                d_tile_transform.translation = transform.translation;
                reset = false;
                slot_processed = true;
                if has_grid {
                    commands.entity(drag.target()).remove::<Placed>();
                } else {
                    commands.entity(entity).insert(Occupied);
                    commands.entity(drag.target()).insert(Placed);
                }
            }
            if tile_processed && slot_processed {
                break;
            }
        }
    }

    if reset {
        d_tile_transform.translation = origin;
    }

    drag_origin.0 = None;
    commands.entity(drag.target()).remove::<Dragging>();
}
