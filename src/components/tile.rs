use bevy::{color::palettes::css::BLACK, prelude::*, text::TextBounds};

#[derive(Component)]
pub struct Tile;

#[derive(Bundle)]
pub struct TileBundle {
    pub tile: Tile,
    pub text: Text2d,
    pub font: TextFont,
    pub color: TextColor,
    pub sprite: Sprite,
    pub transform: Transform,
    pub pickable: Pickable,
}

impl TileBundle {
    pub fn new(
        character: char,
        font: Handle<Font>,
        image: Handle<Image>,
        transform: Transform,
    ) -> Self {
        Self {
            tile: Tile,
            text: Text2d::new(character),
            font: TextFont {
                font,
                font_size: 75.0,
                ..Default::default()
            },
            color: TextColor(BLACK.into()),
            sprite: Sprite::from_image(image),
            transform,
            pickable: Pickable {
                should_block_lower: false,
                is_hoverable: true,
            },
        }
    }
}

#[derive(Bundle)]
pub struct TileScoreBundle {
    pub text: Text2d,
    pub font: TextFont,
    pub color: TextColor,
    pub layout: TextLayout,
    pub bounds: TextBounds,
}

impl TileScoreBundle {
    pub fn new(score: impl Into<String>, font: Handle<Font>) -> Self {
        Self {
            text: Text2d::new(score),
            font: TextFont {
                font,
                font_size: 20.0,
                ..Default::default()
            },
            color: TextColor(BLACK.into()),
            layout: TextLayout::new_with_justify(Justify::Right),
            bounds: TextBounds::from(Vec2::new(89., 95.)),
        }
    }
}
