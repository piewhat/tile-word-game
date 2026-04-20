use bevy::prelude::*;
use bevy::text::TextBounds;

#[derive(Component, Default)]
pub struct Slot(pub Option<Entity>);

#[derive(Component)]
pub struct SwapSlot(pub Entity);

#[derive(Component)]
pub struct SlotUpdate;

#[derive(Bundle, Default)]
pub struct SlotBundle {
    pub slot: Slot,
    pub font: TextFont,
    pub color: TextColor,
    pub layout: TextLayout,
    pub bounds: TextBounds,
    pub sprite: Sprite,
    pub transform: Transform,
    pub pickable: Pickable,
}

impl SlotBundle {
    pub fn new(
        tile: Option<Entity>,
        font: Handle<Font>,
        image: Handle<Image>,
        transform: Transform,
    ) -> Self {
        use bevy::color::palettes::css::BLACK;

        Self {
            slot: Slot(tile),
            font: TextFont {
                font,
                font_size: 20.0,
                ..default()
            },
            color: TextColor(BLACK.into()),
            layout: TextLayout::new_with_justify(Justify::Right),
            bounds: TextBounds::from(Vec2::new(89., 95.)),
            sprite: Sprite::from_image(image),
            transform,
            pickable: Pickable {
                should_block_lower: false,
                is_hoverable: true,
            },
        }
    }
}
