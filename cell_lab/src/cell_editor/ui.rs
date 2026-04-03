use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Color32, Context, CornerRadius, Stroke, Ui, containers::ComboBox},
};
use strum::IntoEnumIterator;

use crate::{
    cell_editor::{
        events::{CellEditorColourMessage, CellEditorSelectedGenomeModeMessage},
        simulation::{CellEditorSimulationClearMessage, CellEditorSimulationStatus},
        state::CellEditorState,
        ui_dialog::{default_genome_mode_dialog, load_or_delete_dialog, save_or_overwrite_dialog},
    },
    game::game_parameters::GameParameters,
    genomes::{CellSplitType, CellType, GenomeBank, GenomeModeId},
};

#[derive(Resource, Default)]
pub struct CellEditorUiStyleApplied(bool);

#[allow(clippy::too_many_lines, clippy::too_many_arguments, clippy::needless_pass_by_value)]
/// # Errors
/// Returns an error if egui ui context cannot be found
pub fn cell_editor_ui_update(
    mut egui_ctx: EguiContexts,
    mut genome_bank: ResMut<GenomeBank>,
    mut state: ResMut<CellEditorState>,
    param: Res<GameParameters>,
    mut sim_status: ResMut<NextState<CellEditorSimulationStatus>>,
    mut cell_editor_style_applied: ResMut<CellEditorUiStyleApplied>,
    mut selected_genome_mode_message_writer: MessageWriter<CellEditorSelectedGenomeModeMessage>,
    mut colour_message_writer: MessageWriter<CellEditorColourMessage>,
    mut simulation_cache_message_writer: MessageWriter<CellEditorSimulationClearMessage>,
) -> Result {
    let ctx = match egui_ctx.ctx_mut() {
        Ok(ctx) => ctx,
        Err(e) => {
            return Err(e)?;
        }
    };

    // Set the cell editor UI style
    set_cell_editor_ui_style(ctx, &mut cell_editor_style_applied.0, &param);

    // Right panel
    egui::SidePanel::right("cell_editor_panel")
        .resizable(false)
        .min_width(param.ui_parameters.cell_editor_panel_width)
        .max_width(param.ui_parameters.cell_editor_panel_width)
        .show(ctx, |ui| {
            // Genome Mode selection
            ui.horizontal(|ui| {
                ui.heading("Cell Editor");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Mode: ");

                    if create_mode_combo_box(&mut state.selected_genome_mode, ui, "selected_mode") {
                        // Selected genome mode was changed
                        selected_genome_mode_message_writer.write(CellEditorSelectedGenomeModeMessage);
                    }
                })
            });

            ui.add_space(param.ui_parameters.separator_spacing);

            ui.horizontal(|ui| {
                // Save button
                if ui.button("Save").clicked() {
                    state.dialogs.open_save_dialog();
                }

                // Load button
                if ui.button("Load").clicked() {
                    state.dialogs.open_load_dialog();
                }

                // Default Genome Mode button
                if ui.button("Replace Mode With Default").clicked() {
                    state.dialogs.open_default_genome_mode_dialog();
                }
            });

            ui.add_space(param.ui_parameters.separator_spacing);
            ui.separator();
            ui.add_space(param.ui_parameters.separator_spacing);

            ui.horizontal(|ui| {
                let mut checked = state.selected_genome_mode == state.get_selected_genome(&genome_bank).initial;
                if ui.checkbox(&mut checked, "Initial Mode").changed() {
                    // Initial genome mode checkbox was clicked
                    if state.get_selected_genome(&genome_bank).initial != state.selected_genome_mode {
                        // Initial genome mode has actually changed
                        state.get_selected_genome_mut(&mut genome_bank).initial = state.selected_genome_mode;

                        // Do an event

                        // Clear the simulation cache
                        simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
                    }
                }

                ui.add_space(param.ui_parameters.subsection_spacing);

                // Cell type selection
                ui.label("Cell Type:");
                ComboBox::from_id_salt("cell_type")
                    .selected_text(format!("{}", state.get_selected_genome_mode(&genome_bank).cell_type))
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(
                                &mut state.get_selected_genome_mode_mut(&mut genome_bank).cell_type,
                                CellType::Phagocyte,
                                CellType::Phagocyte.to_string(),
                            )
                            .changed()
                            || ui
                                .selectable_value(
                                    &mut state.get_selected_genome_mode_mut(&mut genome_bank).cell_type,
                                    CellType::Photocyte,
                                    CellType::Photocyte.to_string(),
                                )
                                .changed()
                        {
                            // Cell type was changed

                            // Clear the simulation cache
                            simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
                        }
                    });
            });

            ui.add_space(param.ui_parameters.separator_spacing);
            ui.separator();
            ui.add_space(param.ui_parameters.separator_spacing);

            // Daughter 1 parameters
            let genome_mode_mut = state.get_selected_genome_mode_mut(&mut genome_bank);
            if create_daughter_subsection(
                ui,
                &mut genome_mode_mut.daughter_genome_modes.0,
                &mut genome_mode_mut.daughter_angles.0,
                0,
                &param,
            ) {
                // Daughter 1 was changed

                // Clear the simulation cache
                simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
            }

            // Daughter 2 parameters
            let genome_mode_mut = state.get_selected_genome_mode_mut(&mut genome_bank);
            if create_daughter_subsection(
                ui,
                &mut genome_mode_mut.daughter_genome_modes.1,
                &mut genome_mode_mut.daughter_angles.1,
                1,
                &param,
            ) {
                // Daughter 2 was changed

                // Clear the simulation cache
                simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
            }

            // Colour selection
            ui.horizontal(|ui| {
                ui.label("Colour: ");

                // Create a colour picker
                if create_colour_edit_ui(ui, &mut state.get_selected_genome_mode_mut(&mut genome_bank).colour) {
                    // Colour was changed
                    colour_message_writer.write(CellEditorColourMessage);

                    // Clear the simulation cache
                    simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
                }
            });

            ui.add_space(param.ui_parameters.separator_spacing);
            ui.separator();
            ui.add_space(param.ui_parameters.separator_spacing);

            // Select "use split age" or "use split energy"
            ui.horizontal(|ui| {
                if ui
                    .radio_value(
                        &mut state.get_selected_genome_mode_mut(&mut genome_bank).split_type,
                        CellSplitType::Energy,
                        "Use Split Energy",
                    )
                    .changed()
                    || ui
                        .radio_value(
                            &mut state.get_selected_genome_mode_mut(&mut genome_bank).split_type,
                            CellSplitType::Age,
                            "Use Split Age",
                        )
                        .changed()
                    || ui
                        .radio_value(
                            &mut state.get_selected_genome_mode_mut(&mut genome_bank).split_type,
                            CellSplitType::Never,
                            "Never Split",
                        )
                        .changed()
                {
                    // Split Type was changed

                    // Clear the simulation cache
                    simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
                }
            });

            // Show different UI depending on use_split_age
            match state.get_selected_genome_mode(&genome_bank).split_type {
                CellSplitType::Energy => {
                    // Split energy parameter
                    ui.horizontal(|ui| {
                        ui.label("Split Energy: ");
                        if ui
                            .add(egui::Slider::new(
                                &mut state.get_selected_genome_mode_mut(&mut genome_bank).split_energy,
                                0.0..=param.cell_parameters.max_energy,
                            ))
                            .changed()
                        {
                            // Split energy was changed

                            // Clear the simulation cache
                            simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
                        }
                    });
                }
                CellSplitType::Age => {
                    // Split age parameter
                    ui.horizontal(|ui| {
                        ui.label("Split Age: ");
                        if ui
                            .add(egui::Slider::new(
                                &mut state.get_selected_genome_mode_mut(&mut genome_bank).split_age,
                                0.0..=param.cell_parameters.max_split_age,
                            ))
                            .changed()
                        {
                            // Split age was changed

                            // Clear the simulation cache
                            simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
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
                        &mut state.get_selected_genome_mode_mut(&mut genome_bank).split_fraction,
                        0.0..=1.0,
                    ))
                    .changed()
                {
                    // Split fraction was changed

                    // Clear the simulation cache
                    simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
                }
            });

            ui.add_space(param.ui_parameters.subsection_spacing);

            // Split angle parameter
            let mut angle_degrees = -state.get_selected_genome_mode(&genome_bank).split_angle.to_degrees();
            ui.horizontal(|ui| {
                ui.label("Split Angle: ");
                if ui.add(egui::Slider::new(&mut angle_degrees, (0.)..=360.)).changed() {
                    // Split angle was changed
                    state.get_selected_genome_mode_mut(&mut genome_bank).split_angle = -angle_degrees.to_radians();

                    // Clear the simulation cache
                    simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
                }
            });

            ui.add_space(param.ui_parameters.subsection_spacing);

            // Split force parameter
            ui.horizontal(|ui| {
                ui.label("Split Force: ");
                if ui
                    .add(egui::Slider::new(
                        &mut state.get_selected_genome_mode_mut(&mut genome_bank).split_force,
                        (0.)..=50.,
                    ))
                    .changed()
                {
                    // Split force was changed

                    // Clear the simulation cache
                    simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
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
                    .add(egui::Slider::new(&mut age, 0.0..=param.cell_editor_mode.max_editor_age).show_value(true))
                    .changed()
                {
                    // Age was changed

                    // Mark the simulation as needing recomputing
                    sim_status.set(CellEditorSimulationStatus::NeedsRecompute);
                }

                // Set the age even if it didnt change to stop editor_age from permanently showing as decreasing/increasing
                state.editor_age.set_age(age);
            });
        });

    let selected_genome = state.get_selected_genome_mut(&mut genome_bank);
    save_or_overwrite_dialog(ctx, &mut state.dialogs, selected_genome);

    load_or_delete_dialog(
        ctx,
        &mut state.dialogs,
        selected_genome,
        &param,
        &mut simulation_cache_message_writer,
    );

    let selected_genome_mode = state.get_selected_genome_mode_mut(&mut genome_bank);
    let selected_genome_mode_id = state.selected_genome_mode;
    default_genome_mode_dialog(
        ctx,
        &mut state.dialogs,
        selected_genome_mode,
        selected_genome_mode_id,
        &param,
        &mut simulation_cache_message_writer,
    );

    Ok(())
}

pub fn set_cell_editor_ui_style(ctx: &mut Context, cell_editor_style_applied: &mut bool, param: &GameParameters) {
    // Set the styles
    if !*cell_editor_style_applied {
        let mut style = (*ctx.style()).clone();
        for font_id in style.text_styles.values_mut() {
            font_id.size *= 1.5; // Scale all fonts
        }
        style.spacing.slider_width = param.ui_parameters.get_cell_editor_slider_width();

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
pub fn create_mode_combo_box(selected_genome_mode: &mut GenomeModeId, ui: &mut Ui, id: impl std::hash::Hash) -> bool {
    let mut changed = false;

    ComboBox::from_id_salt(id)
        .selected_text(format!("{selected_genome_mode}"))
        .show_ui(ui, |ui| {
            // Iterate through the enums
            changed = GenomeModeId::iter()
                .map(|genome_mode| {
                    ui.selectable_value(selected_genome_mode, genome_mode, genome_mode.to_string())
                        .changed()
                })
                .fold(false, |acc, changed| acc | changed);
        });

    changed
}

#[must_use]
pub fn create_daughter_subsection(
    ui: &mut Ui,
    daughter_genome_mode: &mut GenomeModeId,
    daughter_angle: &mut f32,
    daughter_index: usize,
    param: &GameParameters,
) -> bool {
    let mut changed = false;

    // Daughter parameters
    ui.label(format!("Daughter {}: ", daughter_index + 1));
    ui.add_space(param.ui_parameters.subsection_spacing);
    ui.horizontal(|ui| {
        ui.label("Mode: ");
        changed = create_mode_combo_box(daughter_genome_mode, ui, format!("daughter_{daughter_index}_mode"));
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
    ui.add_space(param.ui_parameters.separator_spacing);
    ui.separator();
    ui.add_space(param.ui_parameters.separator_spacing);

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
