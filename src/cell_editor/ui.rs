use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, containers::ComboBox},
};

use crate::{
    cell::{MAX_CELL_AGE, MAX_CELL_ENERGY},
    cell_editor::{
        events::{CellEditorAgeMessage, CellEditorColourMessage, CellEditorSelectedGenomeMessage},
        state::CellEditorState,
    },
    genome::{CellSplitType, CellType},
    ui::{
        SEPARATOR_SPACING, SUBSECTION_SPACING, create_colour_edit_ui, create_daughter_subsection, create_mode_combo_box,
        set_cell_editor_ui_style,
    },
};

const CELL_EDITOR_WIDTH: f32 = 600.;

#[derive(Resource, Default)]
pub struct CellEditorUiStyleApplied(bool);

// TODO
#[allow(clippy::too_many_lines)]
/// # Errors
/// Returns an error if egui ui context cannot be found
pub fn cell_editor_ui_update(
    mut egui_ctx: EguiContexts,
    mut state: ResMut<CellEditorState>,
    mut cell_editor_style_applied: ResMut<CellEditorUiStyleApplied>,
    mut age_message_writer: MessageWriter<CellEditorAgeMessage>,
    mut selected_genome_message_writer: MessageWriter<CellEditorSelectedGenomeMessage>,
    mut colour_message_writer: MessageWriter<CellEditorColourMessage>,
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

            ui.add_space(SEPARATOR_SPACING);
            ui.separator();
            ui.add_space(SEPARATOR_SPACING);

            ui.horizontal(|ui| {
                let mut checked = state.selected_genome == state.genomes.initial;
                if ui.checkbox(&mut checked, "Initial Genome").changed() {
                    // Initial genome checkbox was clicked
                    if state.genomes.initial != state.selected_genome {
                        // Initial genome has actually changed
                        state.genomes.initial = state.selected_genome;

                        // Do an event
                    }
                }

                ui.add_space(SUBSECTION_SPACING);

                // Cell type selection
                ui.label("Cell Type:");
                ComboBox::from_id_salt("cell_type")
                    .selected_text(format!("{}", state.get_selected_genome().cell_type))
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(
                                &mut state.get_selected_genome_mut().cell_type,
                                CellType::Phagocyte,
                                CellType::Phagocyte.to_string(),
                            )
                            .changed()
                            || ui
                                .selectable_value(
                                    &mut state.get_selected_genome_mut().cell_type,
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
            if create_daughter_subsection(ui, &mut state.get_selected_genome_mut().daughter_genomes.0, 0) {
                // Daughter 1 was changed
            }

            // Daughter 2 parameters
            if create_daughter_subsection(ui, &mut state.get_selected_genome_mut().daughter_genomes.1, 1) {
                // Daughter 2 was changed
            }

            // Colour selection
            ui.horizontal(|ui| {
                ui.label("Colour: ");

                // Create a colour picker
                if create_colour_edit_ui(ui, &mut state.get_selected_genome_mut().colour) {
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
                    &mut state.get_selected_genome_mut().split_type,
                    CellSplitType::Energy,
                    "Use Split Energy",
                );
                ui.radio_value(
                    &mut state.get_selected_genome_mut().split_type,
                    CellSplitType::Age,
                    "Use Split Age",
                );
                ui.radio_value(
                    &mut state.get_selected_genome_mut().split_type,
                    CellSplitType::Never,
                    "Never Split",
                );
            });

            // Show different UI depending on use_split_age
            match state.get_selected_genome().split_type {
                CellSplitType::Energy => {
                    // Split energy parameter
                    ui.horizontal(|ui| {
                        ui.label("Split Energy: ");
                        if ui
                            .add(egui::Slider::new(
                                &mut state.get_selected_genome_mut().split_energy,
                                0.0..=MAX_CELL_ENERGY,
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
                                &mut state.get_selected_genome_mut().split_age,
                                0.0..=MAX_CELL_AGE,
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
                        &mut state.get_selected_genome_mut().split_fraction,
                        0.0..=1.0,
                    ))
                    .changed()
                {
                    // Split fraction was changed
                }
            });

            ui.add_space(SUBSECTION_SPACING);

            // Split angle parameter
            let mut angle_degrees = state.get_selected_genome().split_angle.to_degrees();
            ui.horizontal(|ui| {
                ui.label("Split Angle: ");
                if ui.add(egui::Slider::new(&mut angle_degrees, (0.)..=360.)).changed() {
                    // Split angle was changed
                    state.get_selected_genome_mut().split_angle = angle_degrees.to_radians();
                }
            });

            ui.add_space(SUBSECTION_SPACING);

            // Split force parameter
            ui.horizontal(|ui| {
                ui.label("Split Force: ");
                if ui
                    .add(egui::Slider::new(
                        &mut state.get_selected_genome_mut().split_force,
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
                if ui
                    .add(egui::Slider::new(&mut state.age, 0.0..=100.0).show_value(true))
                    .changed()
                {
                    // Age was changed
                    age_message_writer.write(CellEditorAgeMessage);
                }
            });
        });

    Ok(())
}
