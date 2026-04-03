use bevy::prelude::*;

use rand::RngExt;

#[must_use]
pub fn random_vec2(size: Vec2) -> Vec2 {
    let mut rng = rand::rng();

    Vec2::new(rng.random_range(-size.x..size.x), rng.random_range(-size.y..size.y))
}

#[derive(Default, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct SanitisedString(String);

impl SanitisedString {
    #[must_use]
    pub const fn new(string: String) -> Self {
        Self(string)
    }
}

impl std::ops::Deref for SanitisedString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SanitisedString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct SemiSanitisedString(String);

impl SemiSanitisedString {
    #[must_use]
    pub const fn new(string: String) -> Self {
        Self(string)
    }
}

impl std::ops::Deref for SemiSanitisedString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SemiSanitisedString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
