use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Ui, containers::ComboBox},
};

use crate::{
    cell::Cell,
    cell_material::CellMaterial,
    dish::DishMarker,
    genome::{CellType, GENOME_MAX_NUM, Genome, GenomeId},
};

const CELL_EDITOR_WIDTH: f32 = 600.;
const SEPARATOR_SPACING: f32 = 8.;
const SUBSECTION_SPACING: f32 = 4.;

#[derive(Resource, Default)]
pub struct CellEditorState {
    selected_genome: GenomeId,
    age: f32,
    genomes: [Genome; GENOME_MAX_NUM],
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
pub struct UiCustomStyleApplied(bool);

pub fn create_mode_combo_box(
    selected_genome: &mut GenomeId,
    ui: &mut Ui,
    id: impl std::hash::Hash,
) -> bevy_egui::egui::InnerResponse<std::option::Option<()>> {
    ComboBox::from_id_salt(id)
        .selected_text(format!("{selected_genome}"))
        .show_ui(ui, |ui| {
            ui.selectable_value(selected_genome, GenomeId::M1, GenomeId::M1.to_string());
            ui.selectable_value(selected_genome, GenomeId::M2, GenomeId::M2.to_string());
            ui.selectable_value(selected_genome, GenomeId::M3, GenomeId::M3.to_string());
            ui.selectable_value(selected_genome, GenomeId::M4, GenomeId::M4.to_string());
            ui.selectable_value(selected_genome, GenomeId::M5, GenomeId::M5.to_string());
            ui.selectable_value(selected_genome, GenomeId::M6, GenomeId::M6.to_string());
            ui.selectable_value(selected_genome, GenomeId::M7, GenomeId::M7.to_string());
            ui.selectable_value(selected_genome, GenomeId::M8, GenomeId::M8.to_string());
            ui.selectable_value(selected_genome, GenomeId::M9, GenomeId::M9.to_string());
        })
}

// TODO
#[allow(clippy::too_many_lines)]
/// # Errors
/// Returns an error if egui ui context cannot be found
pub fn cell_editor_ui_update(
    mut egui_ctx: EguiContexts,
    mut editor_state: ResMut<CellEditorState>,
    mut ui_style_applied: ResMut<UiCustomStyleApplied>,
) -> Result {
    let ctx = match egui_ctx.ctx_mut() {
        Ok(ctx) => ctx,
        Err(e) => {
            return Err(e)?;
        }
    };

    // Set the styles
    if !ui_style_applied.0 {
        let mut style = (*ctx.style()).clone();
        for font_id in style.text_styles.values_mut() {
            font_id.size *= 1.5; // Scale all fonts
        }
        style.spacing.slider_width = 400.;

        // Colors for sliders
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(0, 180, 10);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(255, 180, 0);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(255, 180, 0);

        // Stroke styles
        style.visuals.widgets.active.fg_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 180, 0));
        style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 180, 0));

        // Set the radius of the knob
        style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(12);
        style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(12);
        style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(12);

        ctx.set_style(style);

        ui_style_applied.0 = true;

        println!("Set the global ui style");
    }

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

                    create_mode_combo_box(&mut editor_state.selected_genome, ui, "selected_mode");
                });
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
                        ui.selectable_value(
                            &mut editor_state.get_selected_genome_mut().cell_type,
                            CellType::Phagocyte,
                            CellType::Phagocyte.to_string(),
                        );
                    });
            });

            ui.add_space(SEPARATOR_SPACING);
            ui.separator();
            ui.add_space(SEPARATOR_SPACING);

            // Daughter 1 parameters
            ui.label("Daughter 1: ");
            ui.add_space(SUBSECTION_SPACING);
            ui.horizontal(|ui| {
                ui.label("Mode: ");
                create_mode_combo_box(
                    &mut editor_state.get_selected_genome_mut().daughter_genomes.0,
                    ui,
                    "daughter_0_mode",
                );
            });

            ui.add_space(SEPARATOR_SPACING);
            ui.separator();
            ui.add_space(SEPARATOR_SPACING);

            // Daughter 2 parameters
            ui.label("Daughter 2: ");
            ui.add_space(SUBSECTION_SPACING);
            ui.horizontal(|ui| {
                ui.label("Mode: ");
                create_mode_combo_box(
                    &mut editor_state.get_selected_genome_mut().daughter_genomes.1,
                    ui,
                    "daughter_1_mode",
                );
            });

            ui.add_space(SEPARATOR_SPACING);
            ui.separator();
            ui.add_space(SEPARATOR_SPACING);

            // Split fraction parameter
            ui.horizontal(|ui| {
                ui.label("Split Fraction: ");
                ui.add(egui::Slider::new(
                    &mut editor_state.get_selected_genome_mut().split_fraction,
                    0.0..=1.0,
                ));
            });

            // Split threshold parameter
            ui.horizontal(|ui| {
                ui.label("Split Threshold: ");
                ui.add(egui::Slider::new(
                    &mut editor_state.get_selected_genome_mut().split_threshold,
                    0.0..=1.0,
                ));
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
                ui.add(egui::Slider::new(&mut editor_state.age, 0.0..=100.0).show_value(true));
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
