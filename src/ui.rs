use bevy::prelude::*;
use bevy_egui::egui::{Color32, ComboBox, Context, CornerRadius, Stroke, Ui};

use crate::genome::GenomeId;

pub const SEPARATOR_SPACING: f32 = 8.;
pub const SUBSECTION_SPACING: f32 = 4.;

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

        println!("Set cell editor UI style globally");
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
pub fn create_daughter_subsection(ui: &mut Ui, daughter_genome: &mut GenomeId, daughter_index: usize) -> bool {
    let mut changed = false;

    // Daughter 1 parameters
    ui.label(format!("Daughter {}: ", daughter_index + 1));
    ui.add_space(SUBSECTION_SPACING);
    ui.horizontal(|ui| {
        ui.label("Mode: ");
        changed = create_mode_combo_box(daughter_genome, ui, format!("daughter_{daughter_index}_mode"));
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
