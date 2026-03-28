use bevy::prelude::*;

// Dish parameters
const DISH_SIZE: Vec2 = Vec2::new(1200., 1200.);
const DISH_COLOUR: Color = Color::linear_rgb(0.2, 0.2, 0.2);

pub struct Dish {
    pub size: Vec2,
}

#[derive(Component)]
pub struct DishMarker;

impl Dish {
    #[must_use]
    pub fn into_bundle(&self) -> impl Bundle {
        (
            Sprite {
                color: DISH_COLOUR,
                custom_size: Some(self.size),
                ..default()
            },
            DishMarker,
        )
    }
}

impl Default for Dish {
    fn default() -> Self {
        Self { size: DISH_SIZE }
    }
}
