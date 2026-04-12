use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    cell_editor::state::CellEditorState,
    game::{game_mode::GameMode, game_parameters::GameParameters},
    genomes::{CellSplitType, CellType, GenomeBank, GenomeModeId},
    ui::{
        ButtonId, CheckboxId, ColourPickerId, ColourPickerMaterial, ComboboxId, RadioId, SliderHueMaterial, SliderId,
        UiPanelType, UiTheme, UiWindowId, spawn_button, spawn_checkbox, spawn_colour_picker, spawn_combobox, spawn_heading,
        spawn_horizontal, spawn_panel, spawn_radio_buttonlike, spawn_semi_separator, spawn_separator, spawn_slider,
        spawn_subheading, window::spawn_floating,
    },
};

#[derive(States, Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UiRebuildState {
    #[default]
    NeedsRebuild,
    Clean,
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
pub fn build_ui(
    mut commands: Commands,

    game_mode: Res<State<GameMode>>,
    needs_rebuild: Res<State<UiRebuildState>>,
    mut next_ui_needs_rebuild: ResMut<NextState<UiRebuildState>>,

    editor_state: Res<CellEditorState>,
    genome_bank: Res<GenomeBank>,
    param: Res<GameParameters>,
    ui_theme: Res<UiTheme>,

    mut colour_picker_materials: ResMut<Assets<ColourPickerMaterial>>,
    mut slider_hue_materials: ResMut<Assets<SliderHueMaterial>>,

    all_ui_windows: Query<Entity, With<UiWindowId>>,

    windows: Query<&Window>,
) {
    // If the UI needs to be rebuilt
    if **needs_rebuild == UiRebuildState::NeedsRebuild {
        // Despawn all the windows
        for window_entity in &all_ui_windows {
            commands.entity(window_entity).despawn();
        }

        // Spawn the correct UI based on the game mode
        match **game_mode {
            GameMode::Simulation => {}
            GameMode::CellEditor => {
                // Spawn the panel
                spawn_cell_editor_panel(
                    &mut commands,
                    &editor_state,
                    &genome_bank,
                    &param,
                    &ui_theme,
                    &mut colour_picker_materials,
                    &mut slider_hue_materials,
                    windows,
                );

                // Spawn the age slider
                spawn_floating(
                    UiWindowId::AgeSliderFloating,
                    Node {
                        margin: UiRect::horizontal(Val::Auto),
                        bottom: px(70),
                        ..default()
                    },
                    &ui_theme,
                    &mut commands,
                    |parent| {
                        spawn_slider(
                            parent,
                            None,
                            SliderId::CellEditorAge,
                            "Age",
                            editor_state.editor_age.get_age(),
                            0.0..=param.cell_editor_mode.max_editor_age,
                            &ui_theme,
                        );
                    },
                );
            }
        }

        // Mark that the rebuild is not needed anymore
        next_ui_needs_rebuild.set(UiRebuildState::Clean);
    }
}

#[allow(clippy::too_many_lines, clippy::needless_pass_by_value, clippy::too_many_arguments)]
pub fn spawn_cell_editor_panel(
    commands: &mut Commands,
    editor_state: &CellEditorState,
    genome_bank: &GenomeBank,
    param: &GameParameters,
    ui_theme: &UiTheme,

    colour_picker_materials: &mut Assets<ColourPickerMaterial>,
    slider_hue_materials: &mut Assets<SliderHueMaterial>,

    windows: Query<&Window>,
) {
    let genome_mode_strings = GenomeModeId::iter()
        .map(|variant| variant.as_ref().to_string())
        .collect::<Vec<_>>();

    // Spawn a panel for the cell editor
    spawn_panel(
        UiWindowId::CellEditorPanel,
        UiPanelType::Right,
        percent(20),
        ui_theme,
        commands,
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
                    spawn_heading(parent, "Genome Editor", ui_theme);

                    // Mode selection
                    spawn_combobox(
                        parent,
                        ComboboxId::SelectedMode,
                        "Mode:",
                        editor_state.selected_genome_mode.into(),
                        &genome_mode_strings,
                        ui_theme,
                    );
                });

            spawn_semi_separator(parent, ui_theme);

            spawn_horizontal(parent, ui_theme, |parent| {
                spawn_button(parent, None, "Save", ButtonId::Save, ui_theme);
                spawn_button(parent, None, "Load", ButtonId::Load, ui_theme);
                spawn_button(
                    parent,
                    None,
                    "Replace Mode With Default",
                    ButtonId::ReplaceModeWithDefault,
                    ui_theme,
                )
            });

            spawn_separator(parent, ui_theme);

            parent
                .spawn(Node {
                    width: percent(100),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    column_gap: ui_theme.window.item_spacing,
                    ..default()
                })
                .with_children(|parent| {
                    spawn_checkbox(
                        parent,
                        CheckboxId::InitialMode,
                        "Initial Mode:",
                        editor_state.get_selected_genome(genome_bank).initial == editor_state.selected_genome_mode,
                        ui_theme,
                    );

                    spawn_combobox(
                        parent,
                        ComboboxId::CellType,
                        "Cell Type:",
                        editor_state.get_selected_genome_mode(genome_bank).cell_type.into(),
                        &CellType::iter()
                            .map(|variant| variant.as_ref().to_string())
                            .collect::<Vec<_>>(),
                        ui_theme,
                    );
                });

            spawn_separator(parent, ui_theme);

            spawn_subheading(parent, "Daughters", ui_theme);

            spawn_semi_separator(parent, ui_theme);

            spawn_subheading(parent, "Daughter 1", ui_theme);

            spawn_combobox(
                parent,
                ComboboxId::Daughter1Mode,
                "Mode:",
                editor_state
                    .get_selected_genome_mode(genome_bank)
                    .daughter_genome_modes
                    .0
                    .into(),
                &genome_mode_strings,
                ui_theme,
            );

            spawn_slider(
                parent,
                None,
                SliderId::Daughter1Angle,
                "Angle",
                -editor_state
                    .get_selected_genome_mode(genome_bank)
                    .daughter_angles
                    .0
                    .to_degrees(),
                0.0..=360.,
                ui_theme,
            );

            spawn_semi_separator(parent, ui_theme);

            spawn_subheading(parent, "Daughter 2", ui_theme);

            spawn_combobox(
                parent,
                ComboboxId::Daughter2Mode,
                "Mode:",
                editor_state
                    .get_selected_genome_mode(genome_bank)
                    .daughter_genome_modes
                    .1
                    .into(),
                &genome_mode_strings,
                ui_theme,
            );

            spawn_slider(
                parent,
                None,
                SliderId::Daughter2Angle,
                "Angle",
                -editor_state
                    .get_selected_genome_mode(genome_bank)
                    .daughter_angles
                    .1
                    .to_degrees(),
                0.0..=360.,
                ui_theme,
            );

            // Have to get window properties
            if let Ok(window) = windows.single() {
                let scale = window.scale_factor();
                let win_size = window.width();

                spawn_separator(parent, ui_theme);

                spawn_subheading(parent, "Colour", ui_theme);

                spawn_colour_picker(
                    parent,
                    editor_state.get_selected_genome_mode(genome_bank).colour,
                    ColourPickerId::SelectedCellColour,
                    ui_theme,
                    scale,
                    win_size,
                    colour_picker_materials,
                    slider_hue_materials,
                );
            }

            spawn_separator(parent, ui_theme);

            spawn_subheading(parent, "Split Parameters", ui_theme);

            spawn_semi_separator(parent, ui_theme);

            spawn_radio_buttonlike(
                parent,
                None,
                RadioId::SplitType,
                "Split Type:",
                editor_state.get_selected_genome_mode(genome_bank).split_type.into(),
                &CellSplitType::iter()
                    .map(|variant| variant.as_ref().to_string())
                    .collect::<Vec<_>>(),
                ui_theme,
            );

            if editor_state.get_selected_genome_mode(genome_bank).split_type == CellSplitType::Energy {
                spawn_slider(
                    parent,
                    None,
                    SliderId::SplitEnergy,
                    "Split Energy:",
                    editor_state.get_selected_genome_mode(genome_bank).split_energy,
                    0.0..=param.cell_parameters.max_energy,
                    ui_theme,
                );
            } else if editor_state.get_selected_genome_mode(genome_bank).split_type == CellSplitType::Age {
                spawn_slider(
                    parent,
                    None,
                    SliderId::SplitAge,
                    "Split Age:",
                    editor_state.get_selected_genome_mode(genome_bank).split_age,
                    0.0..=param.cell_parameters.max_split_age,
                    ui_theme,
                );
            }

            spawn_slider(
                parent,
                None,
                SliderId::SplitFraction,
                "Split Fraction:",
                editor_state.get_selected_genome_mode(genome_bank).split_fraction,
                0.0..=1.0,
                ui_theme,
            );

            spawn_slider(
                parent,
                None,
                SliderId::SplitAngle,
                "Split Angle:",
                -editor_state.get_selected_genome_mode(genome_bank).split_angle.to_degrees(),
                0.0..=360.0,
                ui_theme,
            );

            spawn_slider(
                parent,
                None,
                SliderId::SplitForce,
                "Split Force:",
                editor_state.get_selected_genome_mode(genome_bank).split_force,
                0.0..=60.0,
                ui_theme,
            );

            // spawn_button(parent, "Confirm", ButtonId::ConfirmOverwriteGenome, ui_theme);
        },
    );
}
