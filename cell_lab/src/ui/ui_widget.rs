use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub enum UiElement {
    Button(ButtonType),
}

#[derive(Component, Copy, Clone)]
pub enum ButtonType {
    Save,
    Load,
}
