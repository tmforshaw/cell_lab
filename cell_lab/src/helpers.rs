use bevy::prelude::*;

use rand::RngExt;

#[must_use]
pub fn random_vec2(size: Vec2) -> Vec2 {
    let mut rng = rand::rng();

    Vec2::new(rng.random_range(-size.x..size.x), rng.random_range(-size.y..size.y))
}

#[must_use]
pub fn sanitise_filename(input: &str) -> String {
    let illegal_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*', '.'];

    input
        .trim() // remove leading/trailing whitespace
        .chars()
        .filter(|c| !illegal_chars.contains(c)) // remove illegal characters
        .map(|c| if c.is_whitespace() { '_' } else { c }) // replace spaces with _
        .collect()
}
