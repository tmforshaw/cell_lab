use bevy::prelude::*;

#[derive(Component)]
pub struct DishMarker;

#[derive(Bundle)]
pub struct DishBundle {
    pub sprite: Sprite,
    pub marker: DishMarker,
}

impl DishBundle {
    #[must_use]
    pub const fn new(sprite: Sprite) -> Self {
        Self {
            sprite,
            marker: DishMarker,
        }
    }
}

pub struct Dish {
    pub size: Vec2,
    pub colour: Color,
}

impl Dish {
    #[must_use]
    pub fn new_bundle(size: Vec2, colour: Color) -> DishBundle {
        DishBundle::new(Sprite {
            color: colour,
            custom_size: Some(size),
            ..default()
        })
    }
}
