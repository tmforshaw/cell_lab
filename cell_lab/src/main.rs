#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::while_float)]
#![allow(clippy::assigning_clones)]

use bevy::{input_focus::InputFocus, prelude::*, sprite_render::Material2dPlugin};
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

use crate::{
    cell_editor::{
        events::{
            CellEditorColourMessage, CellEditorInitialGenomeModeMessage, CellEditorSelectedGenomeModeMessage,
            add_selection_borders, cell_editor_colour_message_reader, cell_editor_initial_genome_mode_message_reader,
            cell_editor_selected_genome_mode_message_reader, remove_selection_borders,
        },
        logical_cell::clear_cells,
        simulation::{
            CellEditorSimulationClearMessage, CellEditorSimulationStatus, clear_simulation_cache_message_reader,
            simulate_to_editor_age, spawn_cells_from_simulation,
        },
        state::{CellEditorState, exit_cell_editor_mode, init_cell_editor_mode},
        ui::{CellEditorUiStyleApplied, cell_editor_ui_update},
    },
    cells::{Cell, CellMaterial, SelectionCellMaterial},
    collision::systems::collision_system,
    despawning::apply_pending_despawns,
    game::{game_mode::GameMode, game_parameters::GameParameters},
    genomes::genome::GenomeBank,
    input::{
        cell_editor_mode_keyboard_event_reader, mode_independent_keyboard_event_reader, simulation_mode_keyboard_event_reader,
    },
    simulation::{
        chemical::{Chemical, ChemicalMaterial, ChemicalTimer},
        state::{exit_simulation_mode, init_simulation_mode},
        systems::{
            bound_cells, cell_decay, cells_absorb_chemical, cells_do_meiosis, increment_cell_age, move_cells, spawn_chemicals,
        },
    },
    spatial_partitioning::{
        cell_quadtree::{CellQuadTree, CellQuadTreeDebug, ShowCellQuadTree},
        chemical_quadtree::{ChemicalQuadTree, ChemicalQuadTreeDebug, ShowChemicalQuadTree},
        quadtree::QuadTreeTrait,
        systems::{build_quadtree, visualise_quadtree},
    },
    ui::{
        ButtonEvent, CheckboxEvent, ComboboxEvent, RadioEvent, SliderEvent, TextInputEvent, UiDialogState, UiTheme,
        button_event_reader, button_interaction_system, checkbox_event_reader, checkbox_interaction_system,
        combobox_event_reader, combobox_option_select_system, combobox_text_update_system, combobox_toggle_system,
        open_or_close_dialogs, radio_event_reader, radio_interaction_system, slider_begin_drag_system, slider_drag_system,
        slider_event_reader, slider_interaction_system, slider_release_system, test_panel::spawn_cell_editor_panel,
        text_input_event_reader, text_input_interaction_system, text_input_typing_system,
    },
};

pub mod cell_editor;
pub mod cells;
pub mod collision;
pub mod despawning;
pub mod game;
pub mod genomes;
pub mod helpers;
pub mod input;
pub mod serialisation;
pub mod simulation;
pub mod spatial_partitioning;
pub mod ui;

// TODO need to show that cell spawned even if it dies instantly (When splitting into a tiny cell)
// TODO Show value of slider value as child of the handle when the handle is being moved (Or just to the side)
// TODO Combobox should close its dialog when click is away from the box
// TODO Force UI to update when the values have changed
// TODO Allow clicking just outside of the slider to adjust value
// TODO Text input

#[allow(clippy::too_many_lines)]
fn main() {
    let param = GameParameters::default();
    let game_mode = GameMode::default();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(Material2dPlugin::<CellMaterial>::default())
        .add_plugins(Material2dPlugin::<SelectionCellMaterial>::default())
        .add_plugins(Material2dPlugin::<ChemicalMaterial>::default())
        .insert_resource(CellQuadTree::new_from_parameters(&param, &game_mode))
        .insert_resource(ChemicalQuadTree::new_from_parameters(&param, &game_mode))
        .insert_resource(ChemicalTimer::new_from_parameters(&param))
        .insert_resource(GenomeBank::new_from_parameters(&param))
        .insert_resource(param)
        .insert_state(game_mode)
        .init_state::<CellEditorSimulationStatus>()
        .init_resource::<UiDialogState>()
        .init_resource::<InputFocus>()
        .init_resource::<CellEditorUiStyleApplied>()
        .init_resource::<CellEditorState>()
        .init_resource::<ShowCellQuadTree>()
        .init_resource::<ShowChemicalQuadTree>()
        // UI Events
        .add_message::<ButtonEvent>()
        .add_message::<RadioEvent>()
        .add_message::<CheckboxEvent>()
        .add_message::<SliderEvent>()
        .add_message::<ComboboxEvent>()
        .add_message::<TextInputEvent>()
        // Other Events
        .add_message::<CellEditorInitialGenomeModeMessage>()
        .add_message::<CellEditorSelectedGenomeModeMessage>()
        .add_message::<CellEditorColourMessage>()
        .add_message::<CellEditorSimulationClearMessage>()
        //
        // --------------------- Mode Independent Systems ----------------------
        //
        .add_systems(
            Startup,
            (UiTheme::setup.before(setup), setup, spawn_cell_editor_panel.after(setup)),
        )
        .add_systems(PreUpdate, apply_pending_despawns.run_if(state_changed::<GameMode>)) // Need to do despawning right now when GameMode changes
        .add_systems(
            Update,
            (
                mode_independent_keyboard_event_reader,
                // UI Events
                button_event_reader,
                radio_event_reader,
                checkbox_event_reader,
                slider_event_reader,
                combobox_event_reader,
                text_input_event_reader,
                open_or_close_dialogs,
            ),
        )
        .add_systems(
            PostUpdate,
            (
                apply_pending_despawns,
                // UI Interaction Systems
                button_interaction_system,
                (
                    slider_interaction_system,
                    slider_begin_drag_system,
                    slider_drag_system.after(slider_begin_drag_system),
                    slider_release_system,
                ),
                checkbox_interaction_system,
                radio_interaction_system,
                (
                    combobox_toggle_system,
                    combobox_option_select_system.after(combobox_toggle_system),
                    combobox_text_update_system.after(combobox_option_select_system),
                ),
                (text_input_typing_system, text_input_interaction_system),
            ),
        ) // Despawn after the update in most cases
        //
        // ------------------------- Simulation Mode ---------------------------
        //
        .add_systems(OnEnter(GameMode::Simulation), init_simulation_mode)
        .add_systems(
            Update,
            (
                simulation_mode_keyboard_event_reader,
                increment_cell_age,
                spawn_chemicals,
                move_cells,
                build_quadtree::<CellQuadTree, Cell>,
                build_quadtree::<ChemicalQuadTree, Chemical>,
                cells_absorb_chemical,
                cells_do_meiosis,
                bound_cells,
                collision_system,
                visualise_quadtree::<Entity, CellQuadTree, ShowCellQuadTree, CellQuadTreeDebug>,
                visualise_quadtree::<Entity, ChemicalQuadTree, ShowChemicalQuadTree, ChemicalQuadTreeDebug>,
            )
                .run_if(in_state(GameMode::Simulation)),
        )
        .add_systems(PostUpdate, (cell_decay).run_if(in_state(GameMode::Simulation)))
        .add_systems(OnExit(GameMode::Simulation), exit_simulation_mode)
        //
        // ------------------------- Cell Editor Mode --------------------------
        //
        .add_systems(OnEnter(GameMode::CellEditor), init_cell_editor_mode)
        .add_systems(
            Update,
            (
                cell_editor_mode_keyboard_event_reader,
                cell_editor_initial_genome_mode_message_reader,
                cell_editor_selected_genome_mode_message_reader,
                cell_editor_colour_message_reader,
                clear_simulation_cache_message_reader,
                (
                    clear_cells,
                    simulate_to_editor_age.after(clear_simulation_cache_message_reader),
                    spawn_cells_from_simulation.after(clear_cells).after(simulate_to_editor_age),
                )
                    .run_if(in_state(CellEditorSimulationStatus::NeedsRecompute)),
                remove_selection_borders.after(spawn_cells_from_simulation),
                add_selection_borders.after(remove_selection_borders),
                build_quadtree::<CellQuadTree, Cell>,
                visualise_quadtree::<Entity, CellQuadTree, ShowCellQuadTree, CellQuadTreeDebug>,
            )
                .run_if(in_state(GameMode::CellEditor)),
        )
        .add_systems(
            EguiPrimaryContextPass,
            cell_editor_ui_update.run_if(in_state(GameMode::CellEditor)),
        )
        .add_systems(OnExit(GameMode::CellEditor), exit_cell_editor_mode)
        //
        // ---------------------------------------------------------------------
        //
        .run();
}

// Spawn cells and chemicals
#[allow(clippy::needless_pass_by_value)]
fn setup(mut commands: Commands, mut clear_colour: ResMut<ClearColor>, ui_theme: Res<UiTheme>) {
    // 2D camera
    commands.spawn(Camera2d);

    // Set the clear colour
    clear_colour.0 = ui_theme.background_colour;
}
