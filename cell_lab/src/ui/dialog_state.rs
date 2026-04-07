use std::collections::HashSet;

use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    serialisation::semi_sanitise_filter_map,
    ui::{
        TextInputId, UiTheme, UiWindowId, spawn_button, spawn_heading, spawn_horizontal, spawn_separator, spawn_text_input,
        window::{UiWindowDialog, spawn_dialog},
    },
};

use super::ButtonId;

#[derive(Resource, Debug, Default)]
pub struct UiDialogState {
    pub save: UiSaveDialogState,
    pub load: UiLoadDialogState,
    pub replace_mode_with_default: UiReplaceModeWithDefaultDialogState,
}

#[derive(Debug, Default)]
pub struct UiSaveDialogState {
    open: bool,
    pub selected_genome: Option<usize>,
}

#[derive(Debug, Default)]
pub struct UiLoadDialogState {
    open: bool,
}

#[derive(Debug, Default)]
pub struct UiReplaceModeWithDefaultDialogState {
    open: bool,
}

impl UiDialogState {
    #[must_use]
    pub const fn is_open(&self, window_id: &UiWindowId) -> Option<bool> {
        match window_id {
            UiWindowId::CellEditor => None, // Not a dialog
            UiWindowId::SaveGenomeDialog => Some(self.save.open),
            UiWindowId::LoadGenomeDialog => Some(self.load.open),
            UiWindowId::ReplaceModeWithDefaultDialog => Some(self.replace_mode_with_default.open),
        }
    }

    /// # Panics
    /// Panics if a non-dialog window is attempted to be opened as a dialog
    pub fn open_dialog(&mut self, window_id: &UiWindowId) {
        match window_id {
            UiWindowId::CellEditor => panic!("Tried to open a non-dialog window as a dialog"),
            UiWindowId::SaveGenomeDialog => {
                *self = Self {
                    save: UiSaveDialogState { open: true, ..default() },
                    ..default()
                };
            }
            UiWindowId::LoadGenomeDialog => {
                *self = Self {
                    load: UiLoadDialogState { open: true },
                    ..default()
                };
            }
            UiWindowId::ReplaceModeWithDefaultDialog => {
                *self = Self {
                    replace_mode_with_default: UiReplaceModeWithDefaultDialogState { open: true },
                    ..default()
                };
            }
        }
    }

    /// # Panics
    /// Panics if a non-dialog window is attempted to be closed as a dialog
    pub fn close_dialog(&mut self, window_id: &UiWindowId) {
        match window_id {
            UiWindowId::CellEditor => panic!("Tried to close a non-dialog window as a dialog"),
            UiWindowId::SaveGenomeDialog => {
                *self = Self {
                    save: UiSaveDialogState {
                        open: false,
                        ..default()
                    },
                    ..default()
                };
            }
            UiWindowId::LoadGenomeDialog => {
                *self = Self {
                    load: UiLoadDialogState { open: false },
                    ..default()
                };
            }
            UiWindowId::ReplaceModeWithDefaultDialog => {
                *self = Self {
                    replace_mode_with_default: UiReplaceModeWithDefaultDialogState { open: false },
                    ..default()
                };
            }
        }
    }

    pub fn close_all_dialogs(&mut self) {
        *self = Self::default();
    }

    #[must_use]
    pub fn spawn_dialog(&self, window_id: &UiWindowId) -> Option<fn(&mut Commands, &mut Self, &UiTheme)> {
        match window_id {
            UiWindowId::CellEditor => None, // Not a dialog
            UiWindowId::SaveGenomeDialog => Some(spawn_save_dialog),
            UiWindowId::LoadGenomeDialog => Some(spawn_load_dialog),
            UiWindowId::ReplaceModeWithDefaultDialog => Some(spawn_replace_mode_with_default_dialog),
        }
    }

    #[must_use]
    pub fn spawn_dialog_if_open(&self, window_id: &UiWindowId) -> Option<fn(&mut Commands, &mut Self, &UiTheme)> {
        match self.is_open(window_id) {
            Some(true) => self.spawn_dialog(window_id),
            _ => None,
        }
    }
}

pub fn spawn_save_dialog(commands: &mut Commands, _dialog_state: &mut UiDialogState, ui_theme: &UiTheme) {
    spawn_dialog(UiWindowId::SaveGenomeDialog, ui_theme, commands, |parent| {
        spawn_heading(parent, "Save Genome", ui_theme);

        spawn_separator(parent, ui_theme);

        spawn_horizontal(parent, ui_theme, |parent| {
            spawn_text_input(
                parent,
                TextInputId::SaveFilename,
                "Filename:",
                "",
                Some(semi_sanitise_filter_map),
                ui_theme,
            );

            spawn_button(parent, "Cancel", ButtonId::CloseAllDialogs, ui_theme);

            spawn_separator(parent, ui_theme);

            // TODO Show genomes that already exist so that their names can be copied (Highlighting the one that matches)
        });
    });
}

pub fn spawn_load_dialog(commands: &mut Commands, _dialog_state: &mut UiDialogState, ui_theme: &UiTheme) {
    spawn_dialog(UiWindowId::LoadGenomeDialog, ui_theme, commands, |parent| {
        spawn_heading(parent, "Load Genome", ui_theme);

        spawn_separator(parent, ui_theme);

        spawn_button(parent, "Cancel", ButtonId::CloseAllDialogs, ui_theme);
    });
}

pub fn spawn_replace_mode_with_default_dialog(commands: &mut Commands, _dialog_state: &mut UiDialogState, ui_theme: &UiTheme) {
    spawn_dialog(UiWindowId::ReplaceModeWithDefaultDialog, ui_theme, commands, |parent| {
        spawn_heading(parent, "Replace Current Mode With Default", ui_theme);

        spawn_separator(parent, ui_theme);

        spawn_horizontal(parent, ui_theme, |parent| {
            spawn_button(parent, "Confirm", ButtonId::ConfirmReplaceModeWithDefault, ui_theme);

            spawn_button(parent, "Cancel", ButtonId::CloseAllDialogs, ui_theme);
        });
    });
}

#[allow(clippy::needless_pass_by_value)]
pub fn open_or_close_dialogs(
    mut commands: Commands,
    mut dialog_state: ResMut<UiDialogState>,
    dialogs: Query<(Entity, &UiWindowId), With<UiWindowDialog>>,
    ui_theme: Res<UiTheme>,
) {
    let mut existing = HashSet::new();

    // Close dialogs that shouldn't be open
    for (entity, window_id) in &dialogs {
        // If the dialog is open and exists, add it to existing, if it should not exist then despawn it
        match dialog_state.is_open(window_id) {
            Some(true) => {
                existing.insert(*window_id);
            }
            Some(false) => {
                commands.entity(entity).despawn();
            }
            None => unreachable!(), // Wasn't a dialog
        }
    }

    // Iterate window ID enum to open dialogs that should be open
    for window_id in UiWindowId::iter() {
        // Don't spawn in dialogs that already exist
        if existing.contains(&window_id) {
            continue;
        }

        // Get the spawn function, if the dialog is open
        if let Some(spawn_function) = dialog_state.spawn_dialog_if_open(&window_id) {
            spawn_function(&mut commands, &mut dialog_state, &ui_theme);
        }
    }
}
