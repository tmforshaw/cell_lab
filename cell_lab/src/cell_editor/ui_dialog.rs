use bevy::prelude::*;
use bevy_egui::egui::{self, Context};

use crate::{
    cell_editor::simulation::CellEditorSimulationClearMessage,
    game::game_parameters::GameParameters,
    genomes::{Genome, GenomeMode, GenomeModeId, genome_mode::colour_from_genome_mode_id},
    helpers::SemiSanitisedString,
    serialisation::{
        delete_genome_file, does_genome_exist_in_folder, get_genomes_in_folder, get_genomes_in_folder_underscore_to_spaces,
        read_genome_file, semi_sanitise_filename, write_genome_to_file,
    },
};

#[derive(Default)]
pub struct CellEditorUiDialogState {
    save_dialog_open: bool,
    pub save_filename: SemiSanitisedString,
    pub save_selected_genome: Option<usize>,
    overwrite_dialog_open: bool,
    save_filename_empty_dialog_open: bool,
    load_dialog_open: bool,
    pub load_selected_file: Option<usize>,
    load_default_genome_dialog_open: bool,
    delete_dialog_open: bool,
    delete_file: Option<SemiSanitisedString>,
    default_genome_mode_dialog_open: bool,
}

impl CellEditorUiDialogState {
    #[must_use]
    pub const fn save_dialog_is_open(&self) -> bool {
        self.save_dialog_open
    }

    #[must_use]
    pub const fn overwrite_dialog_is_open(&self) -> bool {
        self.overwrite_dialog_open
    }

    #[must_use]
    pub const fn save_filename_empty_dialog_is_open(&self) -> bool {
        self.save_filename_empty_dialog_open
    }

    #[must_use]
    pub const fn load_dialog_is_open(&self) -> bool {
        self.load_dialog_open
    }

    #[must_use]
    pub const fn load_default_genome_dialog_is_open(&self) -> bool {
        self.load_default_genome_dialog_open
    }

    #[must_use]
    pub const fn delete_dialog_is_open(&self) -> bool {
        self.delete_dialog_open
    }

    #[must_use]
    pub const fn default_genome_mode_dialog_is_open(&self) -> bool {
        self.default_genome_mode_dialog_open
    }

    pub fn open_save_dialog(&mut self) {
        // Open save dialog, everything else gets cleared
        *self = Self {
            save_dialog_open: true,
            ..default()
        };
    }

    pub fn open_overwrite_dialog(&mut self) {
        // Open overwrite dialog, everything except save_filename gets cleared
        *self = Self {
            overwrite_dialog_open: true,
            save_filename: self.save_filename.clone(),
            ..default()
        };
    }

    pub fn open_save_filename_empty_dialog(&mut self) {
        // Open save filename empty dialog, everything except save_filename and selected genome gets cleared
        *self = Self {
            save_filename_empty_dialog_open: true,
            save_filename: self.save_filename.clone(),
            save_selected_genome: self.save_selected_genome,
            ..default()
        };
    }

    pub fn open_load_dialog(&mut self) {
        // Open load dialog, everything else gets cleared
        *self = Self {
            load_dialog_open: true,
            ..default()
        };
    }

    pub fn open_load_default_genome_dialog(&mut self) {
        // Open load default genome dialog, closing load dialog, and keeping load selected file the same
        *self = Self {
            load_default_genome_dialog_open: true,
            load_dialog_open: false,
            load_selected_file: self.load_selected_file,
            ..default()
        };
    }

    pub fn open_delete_dialog(&mut self, delete_file: SemiSanitisedString) {
        // Open delete dialog, set delete_file, close the load dialog, keeping the selected file the same
        *self = Self {
            delete_dialog_open: true,
            delete_file: Some(delete_file),
            load_dialog_open: false,
            load_selected_file: self.load_selected_file,
            ..default()
        };
    }

    pub fn open_default_genome_mode_dialog(&mut self) {
        // Open default genome mode dialog, everything else gets cleared
        *self = Self {
            default_genome_mode_dialog_open: true,
            ..default()
        };
    }

    pub fn close_all_dialogs(&mut self) {
        *self = Self::default();
    }

    pub fn close_save_filename_empty_dialog(&mut self) {
        // Close save filename empty dialog, open the save dialog, keeping the selected filename and selected genome the same
        *self = Self {
            save_filename_empty_dialog_open: false,
            save_dialog_open: true,
            save_filename: self.save_filename.clone(),
            save_selected_genome: self.save_selected_genome,
            ..default()
        };
    }

    pub fn close_load_default_genome_dialog(&mut self) {
        // Close load default genome dialog, open the load dialog, keeping the selected file the same
        *self = Self {
            delete_dialog_open: false,
            load_dialog_open: true,
            load_selected_file: self.load_selected_file,
            ..default()
        };
    }

    pub fn close_delete_dialog(&mut self) {
        // Close delete dialog, open the load dialog, keeping the selected file the same
        *self = Self {
            delete_dialog_open: false,
            load_dialog_open: true,
            load_selected_file: self.load_selected_file,
            ..default()
        };
    }
}

pub fn save_or_overwrite_dialog(ctx: &Context, dialogs: &mut CellEditorUiDialogState, selected_genome: &mut Genome) {
    // If overwrite dialog is open, don't show save dialog
    if dialogs.overwrite_dialog_is_open() {
        egui::Window::new(format!("Overwrite '{}'", *dialogs.save_filename))
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Confirm overwrite
                    if ui.button("Confirm").clicked() {
                        write_genome_to_file(&dialogs.save_filename, selected_genome);

                        // Exit the dialog
                        dialogs.close_all_dialogs();
                    }

                    // Cancel overwrite
                    if ui.button("Cancel").clicked() {
                        // Exit the dialog
                        dialogs.close_all_dialogs();
                    }
                });
            });

    // If save filename empty dialog is open
    } else if dialogs.save_filename_empty_dialog_is_open() {
        egui::Window::new("Can't Save Genome With Empty Filename".to_string())
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    // Close dialog
                    if ui.button("Ok").clicked() {
                        // Exit the dialog
                        dialogs.close_save_filename_empty_dialog();
                    }
                });
            });
    } else {
        // Render save dialog if it is open
        if dialogs.save_dialog_is_open() {
            egui::Window::new("Save Genome")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name of genome: ");
                        if ui.text_edit_singleline(&mut *dialogs.save_filename).changed() {
                            // Semi-sanitise the name so it can be a filename (Leave spaces for now)
                            dialogs.save_filename = semi_sanitise_filename((*dialogs.save_filename).clone());

                            // Check genomes in the genome folder to see if any match the file name
                            if let Some(genomes) = get_genomes_in_folder_underscore_to_spaces()
                                && let Some((i, _)) = genomes
                                    .iter()
                                    .enumerate()
                                    .find(|&(_, genome)| genome == &dialogs.save_filename)
                            {
                                // Set this as the selected genome
                                dialogs.save_selected_genome = Some(i);
                            } else {
                                // Clear the selected genome
                                dialogs.save_selected_genome = None;
                            }
                        }

                        ui.horizontal(|ui| {
                            // Save genome
                            if ui.button("Submit").clicked() {
                                // Check if the file already exists
                                if does_genome_exist_in_folder(&dialogs.save_filename) {
                                    // Open the overwrite dialog
                                    dialogs.open_overwrite_dialog();
                                } else if dialogs.save_filename.trim().is_empty() {
                                    // Filename was empty
                                    dialogs.open_save_filename_empty_dialog();
                                } else {
                                    // Write genome to file
                                    write_genome_to_file(&dialogs.save_filename, selected_genome);

                                    // Exit the dialog
                                    dialogs.close_all_dialogs();
                                }
                            }

                            // Cancel save
                            if ui.button("Cancel").clicked() {
                                // Exit the dialog
                                dialogs.close_all_dialogs();
                            }
                        });
                    });

                    // If there are genomes already saved
                    if let Some(genomes) = get_genomes_in_folder_underscore_to_spaces() {
                        // Iterate genomes and create a selectable value for each
                        if genomes
                            .iter()
                            .enumerate()
                            .map(|(i, genome)| {
                                ui.selectable_value(&mut dialogs.save_selected_genome, Some(i), (**genome).clone())
                                    .changed()
                            })
                            .fold(false, |acc, changed| acc | changed)
                        {
                            // Genome was selected
                            if let Some(genome_id) = dialogs.save_selected_genome {
                                // This shouldn't ever not be true
                                dialogs.save_filename = genomes[genome_id].clone();
                            }
                        }
                    }
                });
        }
    }
}

pub fn load_or_delete_dialog(
    ctx: &Context,
    dialogs: &mut CellEditorUiDialogState,
    selected_genome: &mut Genome,
    param: &GameParameters,
    simulation_cache_message_writer: &mut MessageWriter<CellEditorSimulationClearMessage>,
) {
    // If delete dialog is open, don't show load dialog
    if dialogs.delete_dialog_is_open() {
        // If the delete file is specified
        if let Some(delete_file) = dialogs.delete_file.clone() {
            egui::Window::new(format!("Delete Genome '{}'", *delete_file))
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        // Confirm deletion
                        if ui.button("Confirm").clicked() {
                            delete_genome_file(&delete_file);

                            // Set the selected file to be 0, unless there are no files left
                            if let Some(files) = get_genomes_in_folder() {
                                dialogs.load_selected_file = if files.is_empty() { None } else { Some(0) }
                            }

                            // Exit this dialog
                            dialogs.close_delete_dialog();
                        }

                        // Cancel deletion
                        if ui.button("Cancel").clicked() {
                            // Exit this dialog
                            dialogs.close_delete_dialog();
                        }
                    });
                });
        } else {
            // Exit this dialog (Delete file was not specified)
            dialogs.close_delete_dialog();
        }
    // Load Default Genome dialog is open
    } else if dialogs.load_default_genome_dialog_is_open() {
        egui::Window::new("Overwrite Genome With Default Genome".to_string())
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Confirm overwrite
                    if ui.button("Confirm").clicked() {
                        *selected_genome = Genome::new_from_parameters(param);

                        // Clear the simulation cache
                        simulation_cache_message_writer.write(CellEditorSimulationClearMessage);

                        // Exit all dialogs
                        dialogs.close_all_dialogs();
                    }

                    // Cancel deletion
                    if ui.button("Cancel").clicked() {
                        // Exit this dialog
                        dialogs.close_load_default_genome_dialog();
                    }
                });
            });
    } else {
        // Render load dialog if it is open
        if dialogs.load_dialog_is_open() {
            egui::Window::new("Load Genome")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    if let Some(files) = get_genomes_in_folder_underscore_to_spaces()
                        && !files.is_empty()
                    {
                        let selected_file = dialogs.load_selected_file.unwrap_or(0);
                        dialogs.load_selected_file = Some(selected_file); // Update the selected file to show the selected value

                        // List out all the selectable genomes in a list
                        if files
                            .iter()
                            .enumerate()
                            .map(|(i, file)| {
                                let mut changed = false;

                                ui.horizontal(|ui| {
                                    // Show a selectable value
                                    changed = ui
                                        .selectable_value(&mut dialogs.load_selected_file, Some(i), (**file).clone())
                                        .changed();

                                    // Delete this genome
                                    if ui.button("Delete").clicked() {
                                        // Open a delete dialog for this file
                                        dialogs.open_delete_dialog(files[i].clone());
                                    }
                                });

                                changed
                            })
                            .fold(false, |acc, changed| acc | changed)
                        {
                            // Selected genome was changed
                        }

                        ui.add_space(param.ui_parameters.separator_spacing);
                        ui.separator();
                        ui.add_space(param.ui_parameters.separator_spacing);

                        if ui.button("Load Default Genome").clicked() {
                            // Load default genome was clicked

                            // Confirm loading of default genome
                            dialogs.open_load_default_genome_dialog();
                        }

                        ui.add_space(param.ui_parameters.separator_spacing);
                        ui.separator();
                        ui.add_space(param.ui_parameters.separator_spacing);

                        ui.horizontal(|ui| {
                            // Load genome
                            if ui.button("Load Genome").clicked() {
                                if let Some(genome) = read_genome_file(&files[selected_file]) {
                                    // Set the genome in GenomeBank
                                    *selected_genome = genome;

                                    // Clear the simulation cache
                                    simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
                                }

                                // Exit the dialog
                                dialogs.close_all_dialogs();
                            }

                            // Cancel loading genome
                            if ui.button("Cancel").clicked() {
                                // Exit the dialog
                                dialogs.close_all_dialogs();
                            }
                        });
                    } else {
                        ui.label("No genomes found...");

                        // Close the load dialog
                        if ui.button("Close Dialog").clicked() {
                            dialogs.close_all_dialogs();
                        }
                    }
                });
        }
    }
}

pub fn default_genome_mode_dialog(
    ctx: &Context,
    dialogs: &mut CellEditorUiDialogState,
    selected_genome_mode: &mut GenomeMode,
    selected_genome_mode_id: GenomeModeId,
    param: &GameParameters,
    simulation_cache_message_writer: &mut MessageWriter<CellEditorSimulationClearMessage>,
) {
    // Render default genome mode dialog if it is open
    if dialogs.default_genome_mode_dialog_is_open() {
        egui::Window::new("Replace Current Mode With Default")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Confirm overwrite of genome
                    if ui.button("Confirm").clicked() {
                        // Make a default genome mode, with the correct colour
                        *selected_genome_mode = GenomeMode::new(selected_genome_mode_id);
                        selected_genome_mode.colour = colour_from_genome_mode_id(selected_genome_mode_id, param);

                        // Clear the simulation cache
                        simulation_cache_message_writer.write(CellEditorSimulationClearMessage);

                        // Exit this dialog
                        dialogs.close_all_dialogs();
                    }

                    // Cancel deletion
                    if ui.button("Cancel").clicked() {
                        // Exit this dialog
                        dialogs.close_all_dialogs();
                    }
                });
            });
    }
}
