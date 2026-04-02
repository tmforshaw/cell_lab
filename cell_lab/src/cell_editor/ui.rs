use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Color32, Context, CornerRadius, Stroke, Ui, containers::ComboBox},
};

use crate::{
    cell_editor::{
        events::{CellEditorAgeMessage, CellEditorColourMessage, CellEditorSelectedGenomeMessage, CellEditorSplitAngleMessage},
        state::CellEditorState,
        ui_dialog::CellEditorUiDialogState,
    },
    cells::{CELL_MAX_ENERGY, CELL_MAX_SPLIT_AGE},
    genomes::{CellSplitType, CellType, GenomeBank, GenomeCollection, GenomeId},
    serialisation::{
        delete_genome_bank_file, does_genome_bank_exist_in_folder, get_genome_banks_in_folder, read_genome_bank_file,
        sanitise_filename, write_genome_bank_to_file,
    },
    ui::{SEPARATOR_SPACING, SUBSECTION_SPACING},
};

const CELL_EDITOR_WIDTH: f32 = 600.;
const MAX_EDITOR_AGE: f32 = 25.;

#[derive(Resource, Default)]
pub struct CellEditorUiStyleApplied(bool);

#[allow(clippy::too_many_lines)]
#[allow(clippy::too_many_arguments)]
/// # Errors
/// Returns an error if egui ui context cannot be found
pub fn cell_editor_ui_update(
    mut egui_ctx: EguiContexts,
    mut genome_collection: ResMut<GenomeCollection>,
    mut state: ResMut<CellEditorState>,
    mut dialog_state: ResMut<CellEditorUiDialogState>,
    mut cell_editor_style_applied: ResMut<CellEditorUiStyleApplied>,
    mut age_message_writer: MessageWriter<CellEditorAgeMessage>,
    mut selected_genome_message_writer: MessageWriter<CellEditorSelectedGenomeMessage>,
    mut colour_message_writer: MessageWriter<CellEditorColourMessage>,
    mut split_angle_message_writer: MessageWriter<CellEditorSplitAngleMessage>,
) -> Result {
    let ctx = match egui_ctx.ctx_mut() {
        Ok(ctx) => ctx,
        Err(e) => {
            return Err(e)?;
        }
    };

    // Set the cell editor UI style
    set_cell_editor_ui_style(ctx, &mut cell_editor_style_applied.0);

    // Right panel
    egui::SidePanel::right("cell_editor_panel")
        .resizable(false)
        .min_width(CELL_EDITOR_WIDTH)
        .show(ctx, |ui| {
            // Genome selection
            ui.horizontal(|ui| {
                ui.heading("Cell Editor"); // left-aligned

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Mode: ");

                    if create_mode_combo_box(&mut state.selected_genome, ui, "selected_mode") {
                        // Selected genome was changed
                        selected_genome_message_writer.write(CellEditorSelectedGenomeMessage);
                    }
                })
            });

            ui.add_space(SUBSECTION_SPACING);

            ui.horizontal(|ui| {
                // Save button
                if ui.button("Save").clicked() {
                    dialog_state.open_save_dialog();
                }

                // Load button
                if ui.button("Load").clicked() {
                    dialog_state.open_load_dialog();
                }
            });

            ui.add_space(SEPARATOR_SPACING);
            ui.separator();
            ui.add_space(SEPARATOR_SPACING);

            ui.horizontal(|ui| {
                let mut checked = state.selected_genome == state.get_selected_genome_bank(&genome_collection).initial;
                if ui.checkbox(&mut checked, "Initial Genome").changed() {
                    // Initial genome checkbox was clicked
                    if state.get_selected_genome_bank(&genome_collection).initial != state.selected_genome {
                        // Initial genome has actually changed
                        state.get_selected_genome_bank_mut(&mut genome_collection).initial = state.selected_genome;

                        // Do an event
                    }
                }

                ui.add_space(SUBSECTION_SPACING);

                // Cell type selection
                ui.label("Cell Type:");
                ComboBox::from_id_salt("cell_type")
                    .selected_text(format!("{}", state.get_selected_genome(&genome_collection).cell_type))
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(
                                &mut state.get_selected_genome_mut(&mut genome_collection).cell_type,
                                CellType::Phagocyte,
                                CellType::Phagocyte.to_string(),
                            )
                            .changed()
                            || ui
                                .selectable_value(
                                    &mut state.get_selected_genome_mut(&mut genome_collection).cell_type,
                                    CellType::Photocyte,
                                    CellType::Photocyte.to_string(),
                                )
                                .changed()
                        {
                            // Cell type was changed
                        }
                    });
            });

            ui.add_space(SEPARATOR_SPACING);
            ui.separator();
            ui.add_space(SEPARATOR_SPACING);

            // Daughter 1 parameters
            let genome_collection_mut = state.get_selected_genome_mut(&mut genome_collection);
            if create_daughter_subsection(
                ui,
                &mut genome_collection_mut.daughter_genomes.0,
                &mut genome_collection_mut.daughter_angles.0,
                0,
            ) {
                // Daughter 1 was changed
            }

            // Daughter 2 parameters
            let genome_collection_mut = state.get_selected_genome_mut(&mut genome_collection);
            if create_daughter_subsection(
                ui,
                &mut genome_collection_mut.daughter_genomes.1,
                &mut genome_collection_mut.daughter_angles.1,
                1,
            ) {
                // Daughter 2 was changed
            }

            // Colour selection
            ui.horizontal(|ui| {
                ui.label("Colour: ");

                // Create a colour picker
                if create_colour_edit_ui(ui, &mut state.get_selected_genome_mut(&mut genome_collection).colour) {
                    // Colour was changed
                    colour_message_writer.write(CellEditorColourMessage);
                }
            });

            ui.add_space(SEPARATOR_SPACING);
            ui.separator();
            ui.add_space(SEPARATOR_SPACING);

            // Select "use split age" or "use split energy"
            ui.horizontal(|ui| {
                ui.radio_value(
                    &mut state.get_selected_genome_mut(&mut genome_collection).split_type,
                    CellSplitType::Energy,
                    "Use Split Energy",
                );
                ui.radio_value(
                    &mut state.get_selected_genome_mut(&mut genome_collection).split_type,
                    CellSplitType::Age,
                    "Use Split Age",
                );
                ui.radio_value(
                    &mut state.get_selected_genome_mut(&mut genome_collection).split_type,
                    CellSplitType::Never,
                    "Never Split",
                );
            });

            // Show different UI depending on use_split_age
            match state.get_selected_genome(&genome_collection).split_type {
                CellSplitType::Energy => {
                    // Split energy parameter
                    ui.horizontal(|ui| {
                        ui.label("Split Energy: ");
                        if ui
                            .add(egui::Slider::new(
                                &mut state.get_selected_genome_mut(&mut genome_collection).split_energy,
                                0.0..=CELL_MAX_ENERGY,
                            ))
                            .changed()
                        {
                            // Split energy was changed
                        }
                    });
                }
                CellSplitType::Age => {
                    // Split age parameter
                    ui.horizontal(|ui| {
                        ui.label("Split Age: ");
                        if ui
                            .add(egui::Slider::new(
                                &mut state.get_selected_genome_mut(&mut genome_collection).split_age,
                                0.0..=CELL_MAX_SPLIT_AGE,
                            ))
                            .changed()
                        {
                            // Split age was changed
                        }
                    });
                }
                CellSplitType::Never => {}
            }

            // Split fraction parameter
            ui.horizontal(|ui| {
                ui.label("Split Fraction: ");
                if ui
                    .add(egui::Slider::new(
                        &mut state.get_selected_genome_mut(&mut genome_collection).split_fraction,
                        0.0..=1.0,
                    ))
                    .changed()
                {
                    // Split fraction was changed
                }
            });

            ui.add_space(SUBSECTION_SPACING);

            // Split angle parameter
            let mut angle_degrees = -state.get_selected_genome(&genome_collection).split_angle.to_degrees();
            ui.horizontal(|ui| {
                ui.label("Split Angle: ");
                if ui.add(egui::Slider::new(&mut angle_degrees, (0.)..=360.)).changed() {
                    // Split angle was changed
                    state.get_selected_genome_mut(&mut genome_collection).split_angle = -angle_degrees.to_radians();

                    split_angle_message_writer.write(CellEditorSplitAngleMessage);
                }
            });

            ui.add_space(SUBSECTION_SPACING);

            // Split force parameter
            ui.horizontal(|ui| {
                ui.label("Split Force: ");
                if ui
                    .add(egui::Slider::new(
                        &mut state.get_selected_genome_mut(&mut genome_collection).split_force,
                        (0.)..=50.,
                    ))
                    .changed()
                {
                    // Split force was changed
                }
            });
        });

    // Age slider parameter
    egui::Area::new(egui::Id::new("age_slider_box"))
        .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -80.0])
        .show(ctx, |ui| {
            // Add some space from the bottom
            ui.add_space(10.0); // 10 pixels above the bottom

            ui.horizontal_centered(|ui| {
                ui.label("Age:");

                let mut age = state.editor_age.get_age();
                if ui
                    .add(egui::Slider::new(&mut age, 0.0..=MAX_EDITOR_AGE).show_value(true))
                    .changed()
                {
                    // Age was changed
                    age_message_writer.write(CellEditorAgeMessage);
                }

                // Set the age even if it didnt change to stop editor_age from permanently showing as decreasing/increasing
                state.editor_age.set_age(age);
            });
        });

    save_or_overwrite_dialog(
        ctx,
        &mut dialog_state,
        state.get_selected_genome_bank_mut(&mut genome_collection),
    );

    load_or_delete_dialog(
        ctx,
        &mut dialog_state,
        state.get_selected_genome_bank_mut(&mut genome_collection),
    );

    Ok(())
}

pub fn set_cell_editor_ui_style(ctx: &mut Context, cell_editor_style_applied: &mut bool) {
    // Set the styles
    if !*cell_editor_style_applied {
        let mut style = (*ctx.style()).clone();
        for font_id in style.text_styles.values_mut() {
            font_id.size *= 1.5; // Scale all fonts
        }
        style.spacing.slider_width = 400.;

        // Colors for sliders
        style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(0, 180, 10);
        style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(255, 180, 0);
        style.visuals.widgets.active.bg_fill = Color32::from_rgb(255, 180, 0);

        // Stroke styles
        style.visuals.widgets.active.fg_stroke = Stroke::new(2.0, Color32::from_rgb(255, 180, 0));
        style.visuals.widgets.hovered.fg_stroke = Stroke::new(2.0, Color32::from_rgb(255, 180, 0));

        // Set the radius of the knob
        style.visuals.widgets.active.corner_radius = CornerRadius::same(12);
        style.visuals.widgets.hovered.corner_radius = CornerRadius::same(12);
        style.visuals.widgets.inactive.corner_radius = CornerRadius::same(12);

        ctx.set_style(style);

        *cell_editor_style_applied = true;
    }
}

#[must_use]
pub fn create_mode_combo_box(selected_genome: &mut GenomeId, ui: &mut Ui, id: impl std::hash::Hash) -> bool {
    let mut changed = false;

    ComboBox::from_id_salt(id)
        .selected_text(format!("{selected_genome}"))
        .show_ui(ui, |ui| {
            changed = ui
                .selectable_value(selected_genome, GenomeId::M1, GenomeId::M1.to_string())
                .changed()
                || ui
                    .selectable_value(selected_genome, GenomeId::M2, GenomeId::M2.to_string())
                    .changed()
                || ui
                    .selectable_value(selected_genome, GenomeId::M3, GenomeId::M3.to_string())
                    .changed()
                || ui
                    .selectable_value(selected_genome, GenomeId::M4, GenomeId::M4.to_string())
                    .changed()
                || ui
                    .selectable_value(selected_genome, GenomeId::M5, GenomeId::M5.to_string())
                    .changed()
                || ui
                    .selectable_value(selected_genome, GenomeId::M6, GenomeId::M6.to_string())
                    .changed()
                || ui
                    .selectable_value(selected_genome, GenomeId::M7, GenomeId::M7.to_string())
                    .changed()
                || ui
                    .selectable_value(selected_genome, GenomeId::M8, GenomeId::M8.to_string())
                    .changed()
                || ui
                    .selectable_value(selected_genome, GenomeId::M9, GenomeId::M9.to_string())
                    .changed();
        });

    changed
}

#[must_use]
pub fn create_daughter_subsection(
    ui: &mut Ui,
    daughter_genome: &mut GenomeId,
    daughter_angle: &mut f32,
    daughter_index: usize,
) -> bool {
    let mut changed = false;

    // Daughter parameters
    ui.label(format!("Daughter {}: ", daughter_index + 1));
    ui.add_space(SUBSECTION_SPACING);
    ui.horizontal(|ui| {
        ui.label("Mode: ");
        changed = create_mode_combo_box(daughter_genome, ui, format!("daughter_{daughter_index}_mode"));
    });

    let mut angle_degrees = -daughter_angle.to_degrees();
    ui.horizontal(|ui| {
        ui.label("Angle: ");
        if ui.add(egui::Slider::new(&mut angle_degrees, (0.)..=360.)).changed() {
            // Daughter angle was changed
            *daughter_angle = -angle_degrees.to_radians();

            changed = true;
        }
    });

    // Add Separator
    ui.add_space(SEPARATOR_SPACING);
    ui.separator();
    ui.add_space(SEPARATOR_SPACING);

    changed
}

#[must_use]
pub fn create_colour_edit_ui(ui: &mut Ui, colour: &mut Color) -> bool {
    // Convert Color to Color32
    let col = colour.to_srgba().to_f32_array();
    let mut colour_32 = Color32::from_rgba_premultiplied(
        (col[0] * 255.) as u8,
        (col[1] * 255.) as u8,
        (col[2] * 255.) as u8,
        (col[3] * 255.) as u8,
    );

    // Create the colour picker
    let response = ui.color_edit_button_srgba(&mut colour_32);

    // Convert Color32 to Color
    if response.changed() {
        *colour = Color::srgba_u8(colour_32.r(), colour_32.g(), colour_32.b(), colour_32.a());

        true
    } else {
        false
    }
}

pub fn save_or_overwrite_dialog(
    ctx: &Context,
    dialog_state: &mut CellEditorUiDialogState,
    selected_genome_bank: &mut GenomeBank,
) {
    // If overwrite dialog is open, don't show save dialog
    if dialog_state.overwrite_dialog_is_open() {
        egui::Window::new(format!("Overwrite '{}'", dialog_state.save_filename))
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Confirm overwrite
                    if ui.button("Confirm").clicked() {
                        write_genome_bank_to_file(dialog_state.save_filename.clone(), selected_genome_bank);

                        // Exit the dialog
                        dialog_state.close_all_dialogs();
                    }

                    // Cancel overwrite
                    if ui.button("Cancel").clicked() {
                        // Exit the dialog
                        dialog_state.close_all_dialogs();
                    }
                });
            });
    } else {
        // Render save dialog if it is open
        if dialog_state.save_dialog_is_open() {
            egui::Window::new("Save Genome")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name of genome: ");
                        if ui.text_edit_singleline(&mut dialog_state.save_filename).changed() {
                            // Sanitise the name so it can be a filename
                            dialog_state.save_filename = sanitise_filename(&dialog_state.save_filename);
                        }

                        ui.horizontal(|ui| {
                            // Save genome
                            if ui.button("Submit").clicked() {
                                // Check if the file already exists
                                if does_genome_bank_exist_in_folder(dialog_state.save_filename.clone()) {
                                    // Open the overwrite dialog
                                    dialog_state.open_overwrite_dialog();
                                } else {
                                    // Write genome to file
                                    write_genome_bank_to_file(dialog_state.save_filename.clone(), selected_genome_bank);

                                    // Exit the dialog
                                    dialog_state.close_all_dialogs();
                                }
                            }

                            // Cancel save
                            if ui.button("Cancel").clicked() {
                                // Exit the dialog
                                dialog_state.close_all_dialogs();
                            }
                        })
                    });
                });
        }
    }
}

pub fn load_or_delete_dialog(ctx: &Context, dialog_state: &mut CellEditorUiDialogState, selected_genome_bank: &mut GenomeBank) {
    // If delete dialog is open, don't show load dialog
    if dialog_state.delete_dialog_is_open() {
        // If the delete file is specified
        if let Some(delete_file) = dialog_state.delete_file.clone() {
            egui::Window::new(format!("Delete Genome '{delete_file}'"))
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        // Confirm deletion
                        if ui.button("Confirm").clicked() {
                            delete_genome_bank_file(delete_file);

                            // Set the selected file to be 0, unless there are no files left
                            if let Some(files) = get_genome_banks_in_folder() {
                                dialog_state.load_selected_file = if files.is_empty() { None } else { Some(0) }
                            }

                            // Exit this dialog
                            dialog_state.close_delete_dialog();
                        }

                        // Cancel deletion
                        if ui.button("Cancel").clicked() {
                            // Exit this dialog
                            dialog_state.close_delete_dialog();
                        }
                    });
                });
        } else {
            // Exit this dialog (Delete file was not specified)
            dialog_state.close_delete_dialog();
        }
    } else {
        // Render load dialog if it is open
        if dialog_state.load_dialog_is_open() {
            egui::Window::new("Load Genome")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    if let Some(files) = get_genome_banks_in_folder()
                        && !files.is_empty()
                    {
                        let selected_file = dialog_state.load_selected_file.unwrap_or(0);
                        dialog_state.load_selected_file = Some(selected_file); // Update the selected file to show the selected value

                        // List out all the selectable genomes in a list
                        if files
                            .iter()
                            .enumerate()
                            .map(|(i, file)| {
                                let mut changed = false;

                                ui.horizontal(|ui| {
                                    // Show a selectable value
                                    changed = ui
                                        .selectable_value(&mut dialog_state.load_selected_file, Some(i), file)
                                        .changed();

                                    // Delete this genome
                                    if ui.button("Delete").clicked() {
                                        // Open a delete dialog for this file
                                        dialog_state.open_delete_dialog(files[selected_file].clone());
                                    }
                                });

                                changed
                            })
                            .fold(false, |acc, changed| acc | changed)
                        {
                            // Selected genome bank was changed
                        }

                        ui.add_space(SEPARATOR_SPACING);
                        ui.separator();
                        ui.add_space(SEPARATOR_SPACING);

                        ui.horizontal(|ui| {
                            // Load genome
                            if ui.button("Load Genome").clicked() {
                                if let Some(genome_bank) = read_genome_bank_file(&files[selected_file]) {
                                    // Set the genome bank in GenomeCollection
                                    *selected_genome_bank = genome_bank;
                                }

                                // Exit the dialog
                                dialog_state.close_all_dialogs();
                            }

                            // Cancel loading genome
                            if ui.button("Cancel").clicked() {
                                // Exit the dialog
                                dialog_state.close_all_dialogs();
                            }
                        });
                    } else {
                        ui.label("No genomes found...");

                        // Close the load dialog
                        if ui.button("Close Dialog").clicked() {
                            dialog_state.close_all_dialogs();
                        }
                    }
                });
        }
    }
}
