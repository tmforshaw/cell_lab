use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    cell_editor::state::CellEditorState,
    game::game_parameters::GameParameters,
    genomes::{CellSplitType, CellType, GenomeBank, GenomeModeId},
    ui::{
        ButtonId, CheckboxId, ComboboxId, RadioId, SliderId, UiPanelType, UiTheme, UiWindowId, spawn_button, spawn_checkbox,
        spawn_combobox, spawn_heading, spawn_horizontal, spawn_panel, spawn_radio_buttonlike, spawn_semi_separator,
        spawn_separator, spawn_slider, spawn_subheading,
    },
};

#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn spawn_cell_editor_panel(
    mut commands: Commands,
    editor_state: Res<CellEditorState>,
    genome_bank: Res<GenomeBank>,
    param: Res<GameParameters>,
    ui_theme: Res<UiTheme>,
) {
    let genome_mode_strings = GenomeModeId::iter()
        .map(|variant| variant.as_ref().to_string())
        .collect::<Vec<_>>();

    // Spawn a panel for the cell editor
    spawn_panel(
        UiWindowId::CellEditor,
        UiPanelType::Left,
        percent(20),
        &ui_theme,
        &mut commands,
        |parent| {
            parent
                .spawn((
                    Node {
                        width: percent(100),
                        flex_direction: FlexDirection::Row,
                        align_content: AlignContent::Start,
                        justify_items: JustifyItems::Start,
                        justify_content: JustifyContent::SpaceBetween,
                        column_gap: ui_theme.window.item_spacing,
                        padding: ui_theme.heading_padding,
                        ..default()
                    },
                    BackgroundColor(ui_theme.window.colour_variant),
                ))
                .with_children(|parent| {
                    // Title
                    spawn_heading(parent, "Genome Editor", &ui_theme);

                    // Mode selection
                    spawn_combobox(
                        parent,
                        ComboboxId::SelectedMode,
                        "Mode:",
                        editor_state.selected_genome_mode.into(),
                        &genome_mode_strings,
                        &ui_theme,
                    );
                });

            spawn_semi_separator(parent, &ui_theme);

            spawn_horizontal(parent, &ui_theme, |parent| {
                spawn_button(parent, None, "Save", ButtonId::Save, &ui_theme);
                spawn_button(parent, None, "Load", ButtonId::Load, &ui_theme);
                spawn_button(
                    parent,
                    None,
                    "Replace Mode With Default",
                    ButtonId::ReplaceModeWithDefault,
                    &ui_theme,
                )
            });

            spawn_separator(parent, &ui_theme);

            parent
                .spawn(Node {
                    width: percent(100),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    column_gap: ui_theme.window.item_spacing,
                    ..default()
                })
                .with_children(|parent| {
                    spawn_checkbox(parent, CheckboxId::InitialMode, "Initial Mode:", true, &ui_theme);

                    spawn_combobox(
                        parent,
                        ComboboxId::CellType,
                        "Cell Type:",
                        editor_state.get_selected_genome_mode(&genome_bank).cell_type.into(),
                        &CellType::iter()
                            .map(|variant| variant.as_ref().to_string())
                            .collect::<Vec<_>>(),
                        &ui_theme,
                    );
                });

            spawn_separator(parent, &ui_theme);

            spawn_subheading(parent, "Daughters", &ui_theme);

            spawn_semi_separator(parent, &ui_theme);

            spawn_subheading(parent, "Daughter 1", &ui_theme);

            spawn_combobox(
                parent,
                ComboboxId::Daughter1Mode,
                "Mode:",
                editor_state
                    .get_selected_genome_mode(&genome_bank)
                    .daughter_genome_modes
                    .0
                    .into(),
                &genome_mode_strings,
                &ui_theme,
            );

            spawn_slider(
                parent,
                SliderId::Daughter1Angle,
                "Angle",
                -editor_state
                    .get_selected_genome_mode(&genome_bank)
                    .daughter_angles
                    .0
                    .to_degrees(),
                0.0..=360.,
                &ui_theme,
            );

            spawn_semi_separator(parent, &ui_theme);

            spawn_subheading(parent, "Daughter 2", &ui_theme);

            spawn_combobox(
                parent,
                ComboboxId::Daughter2Mode,
                "Mode:",
                editor_state
                    .get_selected_genome_mode(&genome_bank)
                    .daughter_genome_modes
                    .1
                    .into(),
                &genome_mode_strings,
                &ui_theme,
            );

            spawn_slider(
                parent,
                SliderId::Daughter2Angle,
                "Angle",
                -editor_state
                    .get_selected_genome_mode(&genome_bank)
                    .daughter_angles
                    .1
                    .to_degrees(),
                0.0..=360.,
                &ui_theme,
            );

            spawn_separator(parent, &ui_theme);

            // TODO Colour

            spawn_subheading(parent, "Colour", &ui_theme);

            spawn_separator(parent, &ui_theme);

            spawn_subheading(parent, "Split Parameters", &ui_theme);

            spawn_semi_separator(parent, &ui_theme);

            spawn_radio_buttonlike(
                parent,
                None,
                RadioId::SplitType,
                "Split Type:",
                editor_state.get_selected_genome_mode(&genome_bank).split_type.into(),
                &CellSplitType::iter()
                    .map(|variant| variant.as_ref().to_string())
                    .collect::<Vec<_>>(),
                &ui_theme,
            );

            // TODO Select between showing split energy, age, and neither

            spawn_slider(
                parent,
                SliderId::SplitEnergy,
                "Split Energy:",
                editor_state.get_selected_genome_mode(&genome_bank).split_energy,
                0.0..=param.cell_parameters.max_energy,
                &ui_theme,
            );

            spawn_slider(
                parent,
                SliderId::SplitAge,
                "Split Age:",
                editor_state.get_selected_genome_mode(&genome_bank).split_age,
                0.0..=param.cell_parameters.max_split_age,
                &ui_theme,
            );

            spawn_slider(
                parent,
                SliderId::SplitFraction,
                "Split Fraction:",
                editor_state.get_selected_genome_mode(&genome_bank).split_fraction,
                0.0..=1.0,
                &ui_theme,
            );

            spawn_slider(
                parent,
                SliderId::SplitAngle,
                "Split Angle:",
                -editor_state.get_selected_genome_mode(&genome_bank).split_angle.to_degrees(),
                0.0..=360.0,
                &ui_theme,
            );

            spawn_slider(
                parent,
                SliderId::SplitForce,
                "Split Force:",
                editor_state.get_selected_genome_mode(&genome_bank).split_force,
                0.0..=60.0,
                &ui_theme,
            );

            // spawn_button(parent, "Confirm", ButtonId::ConfirmOverwriteGenome, &ui_theme);
        },
    );
}
