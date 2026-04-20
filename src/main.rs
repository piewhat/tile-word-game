use bevy::{color::palettes::css::*, prelude::*, text::TextBounds};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use fastrand;

mod components;
mod events;
mod resources;
mod systems;

use crate::{
    components::{
        grid::Grid,
        slot::{Slot, SlotBundle, SlotUpdate, SwapSlot},
        tile::{Tile, TileBundle, TileScoreBundle},
    },
    events::{SubmitWord, WordAccepted, WordRejected},
    resources::{Dictionary, GameProgress},
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

#[derive(Component)]
struct ScoreDisplay;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::srgb(1., 1., 1.)))
        .init_resource::<WorldCoords>()
        .init_resource::<DragOrigin>()
        .init_resource::<Dictionary>()
        .init_resource::<GameProgress>()
        .add_observer(validate_submitted_word)
        .add_observer(handle_word_accepted)
        .add_observer(handle_word_rejected)
        .add_systems(Startup, add_tile)
        .add_systems(
            Update,
            (
                cursor,
                check_spacebar_and_reset,
                handle_typing,
                swap_slots,
                update_slots,
                update_score_display,
            ),
        )
        .run();
}

fn add_tile(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, MainCamera));

    // Spawn score display
    commands.spawn((
        Text2d::new("Score: 0"),
        TextFont {
            font: asset_server.load("fonts/Karla-ExtraBold.ttf"),
            font_size: 40.0,
            ..Default::default()
        },
        TextColor(BLACK.into()),
        Transform::from_xyz(0., 330., 0.),
        ScoreDisplay,
    ));

    let mut vowels_needed = 4;
    let mut processed = 0;
    for r in -2..2 {
        for c in -2..2 {
            let random_char = get_random_letter(&mut vowels_needed, 16 - processed);
            processed += 1;
            let score = get_char_score(random_char);
            let tile = commands
                .spawn(TileBundle::new(
                    random_char,
                    asset_server.load("fonts/Karla-ExtraBold.ttf"),
                    asset_server.load("box.png"),
                    Transform::from_xyz((r as f32 + 0.5) * 110., (c as f32 - 1.) * 110., 0.),
                ))
                .with_child(TileScoreBundle::new(
                    score.to_string(),
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
                Text2d::new(format!("\n\n\n\n{}", i + 6)),
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

    commands
        .spawn((
            Sprite::from_image(asset_server.load("skip.png")),
            Transform::from_xyz(275., -330., 0.),
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
        .observe(skip_board);
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

fn skip_board(
    _click: On<Pointer<Click>>,
    mut commands: Commands,
    mut progress: ResMut<GameProgress>,
    mut grid_slots: Query<(&mut Slot, Entity), With<Grid>>,
    mut top_slots: Query<&mut Slot, Without<Grid>>,
    asset_server: Res<AssetServer>,
) {
    progress.score = progress.score.saturating_sub(50);

    // Despawn tiles in top slots
    for mut slot in top_slots.iter_mut() {
        if let Some(tile_entity) = slot.0.take() {
            commands.entity(tile_entity).despawn();
        }
    }

    let count = grid_slots.iter().count();
    let mut vowels_needed = (count / 4).max(1);
    let mut processed = 0;

    // Despawn grid tiles and refill with new ones
    for (mut slot, slot_entity) in grid_slots.iter_mut() {
        if let Some(tile_entity) = slot.0.take() {
            commands.entity(tile_entity).despawn();
        }

        let random_char = get_random_letter(&mut vowels_needed, count - processed);
        processed += 1;
        let score = get_char_score(random_char);
        let tile = commands
            .spawn(TileBundle::new(
                random_char,
                asset_server.load("fonts/Karla-ExtraBold.ttf"),
                asset_server.load("box.png"),
                Transform::default(),
            ))
            .with_child(TileScoreBundle::new(
                score.to_string(),
                asset_server.load("fonts/Roboto-Regular.ttf"),
            ))
            .observe(update_tile_position)
            .observe(tile_startdrag)
            .id();

        slot.0 = Some(tile);
        commands.entity(slot_entity).insert(SlotUpdate);
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

fn update_score_display(
    progress: Res<GameProgress>,
    mut query: Query<&mut Text2d, With<ScoreDisplay>>,
) {
    if progress.is_changed() {
        for mut text in query.iter_mut() {
            text.0 = format!("Score: {}", progress.score);
        }
    }
}

fn validate_submitted_word(
    trigger: On<SubmitWord>,
    dictionary: Res<Dictionary>,
    mut commands: Commands,
) {
    let word = &trigger.event().word;
    if dictionary.words.contains(word) {
        let score: u32 = word.chars().map(get_char_score).sum();
        commands.trigger(WordAccepted {
            word: word.clone(),
            score,
        });
    } else {
        commands.trigger(WordRejected { word: word.clone() });
    }
}

fn handle_word_accepted(
    trigger: On<WordAccepted>,
    mut progress: ResMut<GameProgress>,
    mut commands: Commands,
    mut top_slots: Query<&mut Slot, Without<Grid>>,
    mut grid_slots: Query<(&mut Slot, Entity), With<Grid>>,
    asset_server: Res<AssetServer>,
) {
    let event = trigger.event();
    progress.score += event.score;
    println!(
        "Word Accepted: {}. New score: {}",
        event.word, progress.score
    );

    // Clear top slots and despawn tiles
    for mut slot in top_slots.iter_mut() {
        if let Some(tile_entity) = slot.0.take() {
            commands.entity(tile_entity).despawn();
        }
    }

    let empty_count = grid_slots.iter().filter(|(s, _)| s.0.is_none()).count();
    let mut vowels_needed = (empty_count / 4).max(if empty_count > 0 { 1 } else { 0 });
    let mut processed = 0;

    // Refill empty grid slots with new tiles
    for (mut slot, slot_entity) in grid_slots.iter_mut() {
        if slot.0.is_none() {
            let random_char = get_random_letter(&mut vowels_needed, empty_count - processed);
            processed += 1;
            let score = get_char_score(random_char);
            let tile = commands
                .spawn(TileBundle::new(
                    random_char,
                    asset_server.load("fonts/Karla-ExtraBold.ttf"),
                    asset_server.load("box.png"),
                    Transform::default(),
                ))
                .with_child(TileScoreBundle::new(
                    score.to_string(),
                    asset_server.load("fonts/Roboto-Regular.ttf"),
                ))
                .observe(update_tile_position)
                .observe(tile_startdrag)
                .id();

            slot.0 = Some(tile);
            commands.entity(slot_entity).insert(SlotUpdate);
        }
    }

    if progress.score >= 100 {
        println!("VICTORY REACHED!");
    }
}

fn handle_word_rejected(
    trigger: On<WordRejected>,
    mut commands: Commands,
    top_slots: Query<(&Slot, Entity), Without<Grid>>,
    grid_slots: Query<(&Slot, Entity), With<Grid>>,
) {
    println!("Word Rejected: {}", trigger.event().word);

    let mut empty_grid_slots = grid_slots
        .iter()
        .filter(|(s, _)| s.0.is_none())
        .map(|(_, e)| e)
        .collect::<Vec<_>>();

    for (slot, slot_entity) in top_slots.iter() {
        if slot.0.is_some() {
            if let Some(grid_slot_entity) = empty_grid_slots.pop() {
                commands
                    .entity(slot_entity)
                    .insert(SwapSlot(grid_slot_entity));
            }
        }
    }
}

fn get_char_score(c: char) -> u32 {
    match c.to_ascii_uppercase() {
        'A' | 'E' | 'I' | 'O' | 'U' => 1,
        'J' | 'K' | 'Q' | 'X' | 'Z' => 5,
        _ => 2,
    }
}

fn get_random_letter(vowels_needed: &mut usize, remaining_slots: usize) -> char {
    let vowels = ['A', 'E', 'I', 'O', 'U'];
    let consonants = [
        'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W',
        'X', 'Y', 'Z',
    ];

    let pick_vowel = if *vowels_needed >= remaining_slots {
        true
    } else if *vowels_needed > 0 {
        fastrand::f32() < 0.33
    } else {
        fastrand::f32() < 0.2
    };

    let c = if pick_vowel {
        vowels[fastrand::usize(..vowels.len())]
    } else {
        consonants[fastrand::usize(..consonants.len())]
    };

    let is_vowel = |c: char| matches!(c, 'A' | 'E' | 'I' | 'O' | 'U');
    if is_vowel(c) && *vowels_needed > 0 {
        *vowels_needed = vowels_needed.saturating_sub(1);
    }
    c
}
