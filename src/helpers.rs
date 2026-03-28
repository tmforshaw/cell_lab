use bevy::prelude::*;

use rand::RngExt;

#[must_use]
pub fn random_vec2(size: Vec2) -> Vec2 {
    let mut rng = rand::rng();

    Vec2::new(rng.random_range(-size.x..size.x), rng.random_range(-size.y..size.y))
}
