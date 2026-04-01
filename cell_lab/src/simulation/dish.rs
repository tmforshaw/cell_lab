use bevy::prelude::*;

// Dish parameters
const DISH_COLOUR: Color = Color::linear_rgb(0.2, 0.2, 0.2);

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
}

impl Dish {
    #[must_use]
    pub const fn new(size: Vec2) -> Self {
        Self { size }
    }

    #[must_use]
    pub fn into_bundle(&self) -> DishBundle {
        DishBundle::new(Sprite {
            color: DISH_COLOUR,
            custom_size: Some(self.size),
            ..default()
        })
    }
}
