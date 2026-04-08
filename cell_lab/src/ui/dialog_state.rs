use std::{collections::HashSet, ops::Deref};

use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    helpers::SemiSanitisedString,
    serialisation::{get_genomes_in_folder_underscore_to_spaces, sanitise_filename, semi_sanitise_filter_map},
    ui::{
        RadioId, TextInputId, UiTheme, UiWindowId, spawn_button, spawn_heading, spawn_horizontal, spawn_radio_textlike,
        spawn_semi_separator, spawn_separator, spawn_subheading, spawn_text_input,
        window::{UiWindowDialog, spawn_dialog},
    },
};

use super::ButtonId;

#[derive(Resource, Debug, Default)]
pub struct UiDialogState {
    // Save Dialogs
    pub save: UiSaveDialogState,
    pub overwrite_genome: UiOverwriteGenomeDialogState,
    pub save_filename_is_empty: UiSaveFilenameIsEmptyDialogState,
    // Load Dialogs
    pub load: UiLoadDialogState,
    pub delete: UiDeleteDialogState,
    pub load_default_genome: UiLoadDefaultGenomeDialogState,
    // Replace Mode With Default Dialogs
    pub replace_mode_with_default: UiReplaceModeWithDefaultDialogState,
}

// Save Dialogs

#[derive(Debug, Default)]
pub struct UiSaveDialogState {
    open: bool,
    pub filename: Option<SemiSanitisedString>,
}

#[derive(Debug, Default)]
pub struct UiOverwriteGenomeDialogState {
    open: bool,
}

#[derive(Debug, Default)]
pub struct UiSaveFilenameIsEmptyDialogState {
    open: bool,
}

// Load Dialogs

#[derive(Debug, Default)]
pub struct UiLoadDialogState {
    open: bool,
    pub filename: Option<SemiSanitisedString>,
}

#[derive(Debug, Default)]
pub struct UiDeleteDialogState {
    open: bool,
}

#[derive(Debug, Default)]
pub struct UiLoadDefaultGenomeDialogState {
    open: bool,
}

// Replace Mode With Default Dialogs

#[derive(Debug, Default)]
pub struct UiReplaceModeWithDefaultDialogState {
    open: bool,
}

impl UiDialogState {
    #[must_use]
    pub const fn is_open(&self, window_id: &UiWindowId) -> Option<bool> {
        match window_id {
            UiWindowId::CellEditorPanel | UiWindowId::AgeSliderFloating => None, // Not a dialog
            UiWindowId::SaveGenomeDialog => Some(self.save.open),
            UiWindowId::OverwriteGenomeDialog => Some(self.overwrite_genome.open),
            UiWindowId::SaveFilenameIsEmptyDialog => Some(self.save_filename_is_empty.open),
            UiWindowId::LoadGenomeDialog => Some(self.load.open),
            UiWindowId::DeleteGenomeDialog => Some(self.delete.open),
            UiWindowId::LoadDefaultGenomeDialog => Some(self.load_default_genome.open),
            UiWindowId::ReplaceModeWithDefaultDialog => Some(self.replace_mode_with_default.open),
        }
    }

    /// # Panics
    /// Panics if a non-dialog window is attempted to be opened as a dialog
    pub fn open_dialog(&mut self, window_id: &UiWindowId) {
        match window_id {
            UiWindowId::CellEditorPanel | UiWindowId::AgeSliderFloating => {
                panic!("Tried to open a non-dialog window as a dialog")
            }
            UiWindowId::SaveGenomeDialog => {
                *self = Self {
                    save: UiSaveDialogState { open: true, ..default() },
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
            UiWindowId::LoadGenomeDialog => {
                *self = Self {
                    load: UiLoadDialogState { open: true, ..default() },
                    ..default()
                };
            }
            UiWindowId::DeleteGenomeDialog => {
                *self = Self {
                    delete: UiDeleteDialogState { open: true },
                    load: UiLoadDialogState {
                        open: false,
                        filename: self.load.filename.clone(),
                    },
                    ..default()
                }
            }
            UiWindowId::LoadDefaultGenomeDialog => {
                *self = Self {
                    load_default_genome: UiLoadDefaultGenomeDialogState { open: true },
                    load: UiLoadDialogState {
                        open: false,
                        filename: self.load.filename.clone(),
                    },
                    ..default()
                }
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
            UiWindowId::CellEditorPanel | UiWindowId::AgeSliderFloating => {
                panic!("Tried to close a non-dialog window as a dialog")
            }
            UiWindowId::SaveGenomeDialog => {
                *self = Self {
                    save: UiSaveDialogState {
                        open: false,
                        ..default()
                    },
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
            UiWindowId::LoadGenomeDialog => {
                *self = Self {
                    load: UiLoadDialogState {
                        open: false,
                        ..default()
                    },
                    ..default()
                };
            }
            UiWindowId::DeleteGenomeDialog => {
                *self = Self {
                    delete: UiDeleteDialogState { open: false },
                    load: UiLoadDialogState {
                        open: true,
                        filename: self.load.filename.clone(),
                    },
                    ..default()
                };
            }
            UiWindowId::LoadDefaultGenomeDialog => {
                *self = Self {
                    load_default_genome: UiLoadDefaultGenomeDialogState { open: false },
                    load: UiLoadDialogState {
                        open: true,
                        filename: self.load.filename.clone(),
                    },
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
            UiWindowId::CellEditorPanel | UiWindowId::AgeSliderFloating => None, // Not a dialog
            UiWindowId::SaveGenomeDialog => Some(spawn_save_dialog),
            UiWindowId::OverwriteGenomeDialog => Some(spawn_overwrite_genome_dialog),
            UiWindowId::SaveFilenameIsEmptyDialog => Some(spawn_save_filename_is_empty_dialog),
            UiWindowId::LoadGenomeDialog => Some(spawn_load_dialog),
            UiWindowId::DeleteGenomeDialog => Some(spawn_delete_dialog),
            UiWindowId::LoadDefaultGenomeDialog => Some(spawn_load_default_genome_dialog),
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

// Save Dialogs -----------------------------------------------------------------------------------------------------------------

pub fn spawn_save_dialog(commands: &mut Commands, _dialog_state: &mut UiDialogState, ui_theme: &UiTheme) {
    spawn_dialog(UiWindowId::SaveGenomeDialog, ui_theme, commands, |parent| {
        spawn_heading(parent, "Save Genome", ui_theme);

        spawn_separator(parent, ui_theme);

        let text_input_entity = spawn_horizontal(parent, ui_theme, |parent| {
            let text_input_entity = spawn_text_input(
                parent,
                TextInputId::SaveFilename,
                "Genome Name:",
                "",
                Some(semi_sanitise_filter_map),
                ui_theme,
            );

            spawn_button(parent, text_input_entity, "Submit", ButtonId::SubmitSaveFilename, ui_theme);

            spawn_button(parent, None, "Cancel", ButtonId::CloseAllDialogs, ui_theme);

            text_input_entity
        });

        // Show the saved genomes if they exist
        if let Some(genomes) = get_genomes_in_folder_underscore_to_spaces() {
            spawn_separator(parent, ui_theme);

            // Show the genomes that exist as selectable values
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
                        text_input_entity,
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
            spawn_button(parent, None, "Confirm", ButtonId::ConfirmOverwriteGenome, ui_theme);

            spawn_button(parent, None, "Cancel", ButtonId::CloseOverwriteGenomeDialog, ui_theme)
        });
    });
}

pub fn spawn_save_filename_is_empty_dialog(commands: &mut Commands, _dialog_state: &mut UiDialogState, ui_theme: &UiTheme) {
    spawn_dialog(UiWindowId::SaveFilenameIsEmptyDialog, ui_theme, commands, |parent| {
        spawn_heading(parent, "Could Not Save Genome With Empty Name", ui_theme);

        spawn_separator(parent, ui_theme);

        spawn_button(parent, None, "Ok", ButtonId::CloseSaveFilenameEmptyDialog, ui_theme);
    });
}

// ------------------------------------------------------------------------------------------------------------------------------

// Load Dialogs -----------------------------------------------------------------------------------------------------------------

pub fn spawn_load_dialog(commands: &mut Commands, _dialog_state: &mut UiDialogState, ui_theme: &UiTheme) {
    spawn_dialog(UiWindowId::LoadGenomeDialog, ui_theme, commands, |parent| {
        spawn_heading(parent, "Load Genome", ui_theme);

        spawn_separator(parent, ui_theme);

        // Show the saved genomes if they exist
        if let Some(genomes) = get_genomes_in_folder_underscore_to_spaces() {
            // Show the genomes that exist as selectable values
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
                        None,
                        RadioId::LoadFileNames,
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

            spawn_semi_separator(parent, ui_theme);

            spawn_button(
                parent,
                None,
                "Delete Selected Genome",
                ButtonId::DeleteSelectedGenome,
                ui_theme,
            );
        } else {
            spawn_subheading(parent, "There Are No Genomes Saved...", ui_theme);
        }

        spawn_separator(parent, ui_theme);

        spawn_button(parent, None, "Load Default Genome", ButtonId::LoadDefaultGenome, ui_theme);

        spawn_semi_separator(parent, ui_theme);

        spawn_horizontal(parent, ui_theme, |parent| {
            spawn_button(parent, None, "Load Genome", ButtonId::ConfirmLoadGenome, ui_theme);
            spawn_button(parent, None, "Cancel", ButtonId::CloseAllDialogs, ui_theme)
        });
    });
}

pub fn spawn_delete_dialog(commands: &mut Commands, dialog_state: &mut UiDialogState, ui_theme: &UiTheme) {
    if let Some(filename) = dialog_state.load.filename.clone() {
        spawn_dialog(UiWindowId::DeleteGenomeDialog, ui_theme, commands, |parent| {
            spawn_heading(parent, format!("Delete Genome '{}'", (*filename).clone()), ui_theme);

            spawn_separator(parent, ui_theme);

            spawn_button(parent, None, "Confirm", ButtonId::ConfirmDeleteGenome, ui_theme);

            spawn_button(parent, None, "Cancel", ButtonId::CloseDeleteDialog, ui_theme);
        });
    }
}

pub fn spawn_load_default_genome_dialog(commands: &mut Commands, _dialog_state: &mut UiDialogState, ui_theme: &UiTheme) {
    spawn_dialog(UiWindowId::LoadDefaultGenomeDialog, ui_theme, commands, |parent| {
        spawn_heading(parent, "Overwrite Genome With Default", ui_theme);

        spawn_separator(parent, ui_theme);

        spawn_button(parent, None, "Confirm", ButtonId::ConfirmLoadDefaultGenome, ui_theme);

        spawn_button(parent, None, "Cancel", ButtonId::CloseLoadDefaultGenome, ui_theme);
    });
}

// ------------------------------------------------------------------------------------------------------------------------------

pub fn spawn_replace_mode_with_default_dialog(commands: &mut Commands, _dialog_state: &mut UiDialogState, ui_theme: &UiTheme) {
    spawn_dialog(UiWindowId::ReplaceModeWithDefaultDialog, ui_theme, commands, |parent| {
        spawn_heading(parent, "Replace Current Mode With Default", ui_theme);

        spawn_separator(parent, ui_theme);

        spawn_horizontal(parent, ui_theme, |parent| {
            spawn_button(parent, None, "Confirm", ButtonId::ConfirmReplaceModeWithDefault, ui_theme);

            spawn_button(parent, None, "Cancel", ButtonId::CloseAllDialogs, ui_theme)
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
