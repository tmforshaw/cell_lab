use std::{collections::HashSet, ops::Deref};

use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    helpers::SemiSanitisedString,
    serialisation::{get_genomes_in_folder_underscore_to_spaces, sanitise_filename, semi_sanitise_filter_map},
    ui::{
        RadioId, TextInputId, UiTheme, UiWindowId, spawn_button, spawn_heading, spawn_horizontal, spawn_radio_textlike,
        spawn_separator, spawn_text_input,
        window::{UiWindowDialog, spawn_dialog},
    },
};

use super::ButtonId;

#[derive(Resource, Debug, Default)]
pub struct UiDialogState {
    pub save: UiSaveDialogState,
    pub load: UiLoadDialogState,
    pub replace_mode_with_default: UiReplaceModeWithDefaultDialogState,
    pub overwrite_genome: UiOverwriteGenomeDialogState,
    pub save_filename_is_empty: UiSaveFilenameIsEmptyDialogState,
}

#[derive(Debug, Default)]
pub struct UiSaveDialogState {
    open: bool,
    pub filename: Option<SemiSanitisedString>,
}

#[derive(Debug, Default)]
pub struct UiLoadDialogState {
    open: bool,
}

#[derive(Debug, Default)]
pub struct UiReplaceModeWithDefaultDialogState {
    open: bool,
}

#[derive(Debug, Default)]
pub struct UiOverwriteGenomeDialogState {
    open: bool,
}

#[derive(Debug, Default)]
pub struct UiSaveFilenameIsEmptyDialogState {
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
            UiWindowId::OverwriteGenomeDialog => Some(self.overwrite_genome.open),
            UiWindowId::SaveFilenameIsEmptyDialog => Some(self.save_filename_is_empty.open),
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
            UiWindowId::OverwriteGenomeDialog => {
                // Close all dialogs, but keep save.selected genome the same
                *self = Self {
                    overwrite_genome: UiOverwriteGenomeDialogState { open: true },
                    save: UiSaveDialogState {
                        open: false,
                        filename: self.save.filename.clone(),
                    },
                    ..default()
                };
            }
            UiWindowId::SaveFilenameIsEmptyDialog => {
                *self = Self {
                    save_filename_is_empty: UiSaveFilenameIsEmptyDialogState { open: true },
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
            UiWindowId::OverwriteGenomeDialog => {
                *self = Self {
                    overwrite_genome: UiOverwriteGenomeDialogState { open: false },
                    save: UiSaveDialogState {
                        open: true,
                        filename: self.save.filename.clone(),
                    },
                    ..default()
                };
            }
            UiWindowId::SaveFilenameIsEmptyDialog => {
                *self = Self {
                    save_filename_is_empty: UiSaveFilenameIsEmptyDialogState { open: false },
                    save: UiSaveDialogState { open: true, ..default() },
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
            UiWindowId::OverwriteGenomeDialog => Some(spawn_overwrite_genome_dialog),
            UiWindowId::SaveFilenameIsEmptyDialog => Some(spawn_save_filename_is_empty_dialog),
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

// Save Dialogs -----------------------------------------------------------------------------------------------------------------

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
        });

        // Show the saved genomes if they exist
        if let Some(genomes) = get_genomes_in_folder_underscore_to_spaces() {
            parent
                .spawn(Node {
                    width: percent(100),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    spawn_radio_textlike(
                        parent,
                        RadioId::SaveFileNames,
                        "",
                        None,
                        &genomes
                            .iter()
                            .map(Deref::deref)
                            .map(sanitise_filename)
                            .map(|sanitised| (*sanitised).clone())
                            .collect::<Vec<_>>(),
                        ui_theme,
                    );
                });
        }
    });
}

pub fn spawn_overwrite_genome_dialog(commands: &mut Commands, dialog_state: &mut UiDialogState, ui_theme: &UiTheme) {
    spawn_dialog(UiWindowId::OverwriteGenomeDialog, ui_theme, commands, |parent| {
        let Some(selected_genome) = dialog_state.save.filename.clone() else {
            eprintln!("Could not convert save filename to SemiSanitisedString");
            return;
        };

        spawn_heading(parent, format!("Overwrite Genome: '{}'", *selected_genome), ui_theme);

        spawn_separator(parent, ui_theme);

        spawn_horizontal(parent, ui_theme, |parent| {
            spawn_button(parent, "Confirm", ButtonId::ConfirmOverwriteGenome, ui_theme);

            spawn_button(parent, "Cancel", ButtonId::CloseOverwriteGenomeDialog, ui_theme);
        });
    });
}

pub fn spawn_save_filename_is_empty_dialog(commands: &mut Commands, _dialog_state: &mut UiDialogState, ui_theme: &UiTheme) {
    spawn_dialog(UiWindowId::SaveFilenameIsEmptyDialog, ui_theme, commands, |parent| {
        spawn_heading(parent, "Could Not Save Genome With Empty Name", ui_theme);

        spawn_separator(parent, ui_theme);

        spawn_button(parent, "Ok", ButtonId::CloseSaveFilenameEmptyDialog, ui_theme);
    });
}

// ------------------------------------------------------------------------------------------------------------------------------

// Load Dialogs -----------------------------------------------------------------------------------------------------------------

pub fn spawn_load_dialog(commands: &mut Commands, _dialog_state: &mut UiDialogState, ui_theme: &UiTheme) {
    spawn_dialog(UiWindowId::LoadGenomeDialog, ui_theme, commands, |parent| {
        spawn_heading(parent, "Load Genome", ui_theme);

        spawn_separator(parent, ui_theme);

        spawn_button(parent, "Cancel", ButtonId::CloseAllDialogs, ui_theme);
    });
}

// ------------------------------------------------------------------------------------------------------------------------------

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
