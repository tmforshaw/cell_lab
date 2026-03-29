use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, containers::ComboBox},
};

use crate::{
    cell::Cell,
    cell_editor_events::{CellEditorMessage, CellEditorParameter},
    cell_material::CellMaterial,
    dish::DishMarker,
    genome::{CellType, GENOME_MAX_NUM, Genome, GenomeId},
    ui::{
        SEPARATOR_SPACING, SUBSECTION_SPACING, create_colour_edit_ui, create_daughter_subsection, create_mode_combo_box,
        set_cell_editor_ui_style,
    },
};

const CELL_EDITOR_WIDTH: f32 = 600.;

#[derive(Resource)]
pub struct CellEditorState {
    selected_genome: GenomeId,
    age: f32,
    genomes: [Genome; GENOME_MAX_NUM],
}

impl Default for CellEditorState {
    fn default() -> Self {
        Self {
            genomes: std::array::from_fn(|i| Genome::new(i.into())),
            selected_genome: GenomeId::default(),
            age: 0.,
        }
    }
}

impl CellEditorState {
    #[must_use]
    pub fn get_selected_genome(&self) -> &Genome {
        &self.genomes[Into::<usize>::into(self.selected_genome)]
    }

    #[must_use]
    pub fn get_selected_genome_mut(&mut self) -> &mut Genome {
        &mut self.genomes[Into::<usize>::into(self.selected_genome)]
    }
}

#[derive(Resource, Default)]
pub struct CellEditorUiStyleApplied(bool);

// TODO
#[allow(clippy::too_many_lines)]
/// # Errors
/// Returns an error if egui ui context cannot be found
pub fn cell_editor_ui_update(
    mut egui_ctx: EguiContexts,
    mut editor_state: ResMut<CellEditorState>,
    mut cell_editor_style_applied: ResMut<CellEditorUiStyleApplied>,
    mut cell_editor_message_writer: MessageWriter<CellEditorMessage>,
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

                    if create_mode_combo_box(&mut editor_state.selected_genome, ui, "selected_mode") {
                        // Selected genome was changed
                        cell_editor_message_writer.write(CellEditorMessage {
                            param: CellEditorParameter::SelectedGenome,
                        });
                    }
                })
            });

            ui.add_space(SEPARATOR_SPACING);
            ui.separator();
            ui.add_space(SEPARATOR_SPACING);

            // Cell type selection
            ui.horizontal(|ui| {
                ui.label("Cell Type:");
                ComboBox::from_id_salt("cell_type")
                    .selected_text(format!("{}", editor_state.get_selected_genome().cell_type))
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(
                                &mut editor_state.get_selected_genome_mut().cell_type,
                                CellType::Phagocyte,
                                CellType::Phagocyte.to_string(),
                            )
                            .changed()
                            || ui
                                .selectable_value(
                                    &mut editor_state.get_selected_genome_mut().cell_type,
                                    CellType::Photocyte,
                                    CellType::Photocyte.to_string(),
                                )
                                .changed()
                        {
                            // Cell type was changed
                            cell_editor_message_writer.write(CellEditorMessage {
                                param: CellEditorParameter::CellType,
                            });
                        }
                    });
            });

            ui.add_space(SEPARATOR_SPACING);
            ui.separator();
            ui.add_space(SEPARATOR_SPACING);

            // Daughter 1 parameters
            if create_daughter_subsection(ui, &mut editor_state.get_selected_genome_mut().daughter_genomes.0, 0) {
                // Daughter 1 was changed
                cell_editor_message_writer.write(CellEditorMessage {
                    param: CellEditorParameter::Daughter1Mode,
                });
            }

            // Daughter 2 parameters
            if create_daughter_subsection(ui, &mut editor_state.get_selected_genome_mut().daughter_genomes.1, 1) {
                // Daughter 2 was changed
                cell_editor_message_writer.write(CellEditorMessage {
                    param: CellEditorParameter::Daughter2Mode,
                });
            }

            // Colour selection
            ui.horizontal(|ui| {
                ui.label("Colour: ");

                // Create a colour picker
                if create_colour_edit_ui(ui, &mut editor_state.get_selected_genome_mut().colour) {
                    // Colour was changed
                    cell_editor_message_writer.write(CellEditorMessage {
                        param: CellEditorParameter::Colour,
                    });
                }
            });

            ui.add_space(SUBSECTION_SPACING);

            // Split fraction parameter
            ui.horizontal(|ui| {
                ui.label("Split Fraction: ");
                if ui
                    .add(egui::Slider::new(
                        &mut editor_state.get_selected_genome_mut().split_fraction,
                        0.0..=1.0,
                    ))
                    .changed()
                {
                    // Split fraction was changed
                    cell_editor_message_writer.write(CellEditorMessage {
                        param: CellEditorParameter::SplitFraction,
                    });
                }
            });

            ui.add_space(SUBSECTION_SPACING);

            // Split threshold parameter
            ui.horizontal(|ui| {
                ui.label("Split Threshold: ");
                if ui
                    .add(egui::Slider::new(
                        &mut editor_state.get_selected_genome_mut().split_threshold,
                        0.0..=1.0,
                    ))
                    .changed()
                {
                    // Split threshold was changed
                    cell_editor_message_writer.write(CellEditorMessage {
                        param: CellEditorParameter::SplitThreshold,
                    });
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
                    .add(egui::Slider::new(&mut editor_state.age, 0.0..=100.0).show_value(true))
                    .changed()
                {
                    // Age was changed
                    cell_editor_message_writer.write(CellEditorMessage {
                        param: CellEditorParameter::Age,
                    });
                }
            });
        });

    Ok(())
}

// ------------------------- Cell Editor Mode --------------------------

pub fn init_cell_editor_mode(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    // Spawn bacground
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.2, 0.2, 0.2),
            custom_size: Some(Vec2::splat(1200.)),
            ..default()
        },
        Transform::from_xyz(0., 0., 0.),
        DishMarker,
    ));

    // Spawn a default cell
    commands.spawn(Cell::new_bundle(
        100.,
        Vec2::ZERO,
        Vec2::ZERO,
        Color::linear_rgb(0.5, 1.0, 0.5),
        &mut meshes,
        &mut materials,
    ));
}

pub fn exit_cell_editor_mode(mut commands: Commands, dishes: Query<Entity, With<DishMarker>>, cells: Query<Entity, With<Cell>>) {
    for entity in dishes {
        commands.entity(entity).despawn();
    }

    for entity in cells {
        commands.entity(entity).despawn();
    }
}

// ---------------------------------------------------------------------
