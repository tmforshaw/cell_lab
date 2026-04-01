#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_lossless)]

use bevy::{prelude::*, sprite_render::Material2dPlugin};
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use bevy_prototype_lyon::plugin::ShapePlugin;

use crate::{
    cell_editor::{
        drawing::draw_cell_info,
        events::{
            CellEditorAgeMessage, CellEditorColourMessage, CellEditorInitialGenomeMessage, CellEditorSelectedGenomeMessage,
            CellEditorSplitAngleMessage, add_selection_borders, cell_editor_age_message_reader,
            cell_editor_colour_message_reader, cell_editor_initial_genome_message_reader,
            cell_editor_selected_genome_message_reader, cell_editor_split_angle_message_reader, remove_selection_borders,
        },
        state::{CellEditorState, exit_cell_editor_mode, init_cell_editor_mode},
        systems::{remove_negative_aged_cells, reverse_splits, split_cells},
        ui::{CellEditorUiStyleApplied, cell_editor_ui_update},
    },
    cells::cell_material::CellMaterial,
    collision::systems::collision_system,
    despawning::apply_pending_despawns,
    game_mode::GameMode,
    genomes::genome_bank::GenomeCollection,
    input::{cell_editor_mode_keyboard_event_reader, simulation_mode_keyboard_event_reader},
    simulation::{
        chemical::{ChemicalMaterial, ChemicalTimer},
        state::{SimulationState, exit_simulation_mode, init_simulation_mode},
        systems::{
            bound_cells, cell_decay, cells_absorb_chemical, cells_do_meiosis, increment_cell_age, move_cells, spawn_chemicals,
        },
    },
    spatial_partitioning::cell_quadtree::{CellQuadTree, ShowCellQuadTree, visualize_cell_quadtree},
};

// TODO Collision and bound check in cell editor messes up the time reversal

pub mod cell_editor;
pub mod cells;
pub mod collision;
pub mod despawning;
pub mod game_mode;
pub mod genomes;
pub mod helpers;
pub mod input;
pub mod simulation;
pub mod spatial_partitioning;
pub mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(ShapePlugin)
        .add_plugins(Material2dPlugin::<CellMaterial>::default())
        .add_plugins(Material2dPlugin::<ChemicalMaterial>::default())
        // .init_state::<GameMode>()
        .insert_state(GameMode::CellEditor)
        .init_resource::<GenomeCollection>()
        .init_resource::<SimulationState>()
        .init_resource::<ChemicalTimer>()
        .init_resource::<CellEditorUiStyleApplied>()
        .init_resource::<CellEditorState>()
        .init_resource::<CellQuadTree>()
        .init_resource::<ShowCellQuadTree>()
        .add_message::<CellEditorInitialGenomeMessage>()
        .add_message::<CellEditorAgeMessage>()
        .add_message::<CellEditorSelectedGenomeMessage>()
        .add_message::<CellEditorColourMessage>()
        .add_message::<CellEditorSplitAngleMessage>()
        //
        // --------------------------- All Systems ----------------------------
        //
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, apply_pending_despawns.run_if(state_changed::<GameMode>)) // Need to do despawning right now when GameMode changes
        .add_systems(PostUpdate, apply_pending_despawns) // Despawn after the update in most cases
        //
        // -------------------------- Simulation Mode -------------------------
        //
        .add_systems(OnEnter(GameMode::Simulation), init_simulation_mode)
        .add_systems(
            Update,
            (
                simulation_mode_keyboard_event_reader,
                increment_cell_age,
                spawn_chemicals,
                move_cells,
                bound_cells,
                collision_system,
                cells_absorb_chemical,
                cells_do_meiosis,
                visualize_cell_quadtree,
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
                collision_system,
                bound_cells,
                cell_editor_mode_keyboard_event_reader,
                cell_editor_initial_genome_message_reader,
                cell_editor_age_message_reader,
                cell_editor_selected_genome_message_reader,
                cell_editor_colour_message_reader,
                cell_editor_split_angle_message_reader,
                remove_selection_borders,
                add_selection_borders,
                draw_cell_info,
                split_cells,
                remove_negative_aged_cells,
                reverse_splits,
                visualize_cell_quadtree,
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
fn setup(mut commands: Commands) {
    // 2D camera
    commands.spawn(Camera2d);
}
