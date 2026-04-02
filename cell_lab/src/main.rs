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
        systems::{modify_cell_energy, remove_low_energy_cells, remove_negative_aged_cells, reverse_splits, split_cells},
        ui::{CellEditorUiStyleApplied, cell_editor_ui_update},
    },
    cells::{Cell, CellMaterial, SelectionCellMaterial},
    collision::systems::collision_system,
    despawning::apply_pending_despawns,
    game_mode::GameMode,
    genomes::genome_bank::GenomeCollection,
    input::{
        cell_editor_mode_keyboard_event_reader, mode_independent_keyboard_event_reader, simulation_mode_keyboard_event_reader,
    },
    simulation::{
        chemical::{Chemical, ChemicalMaterial, ChemicalTimer},
        state::{SimulationState, exit_simulation_mode, init_simulation_mode},
        systems::{
            bound_cells, cell_decay, cells_absorb_chemical, cells_do_meiosis, increment_cell_age, move_cells, spawn_chemicals,
        },
    },
    spatial_partitioning::{
        cell_quadtree::{CellQuadTree, CellQuadTreeDebug, ShowCellQuadTree},
        chemical_quadtree::{ChemicalQuadTree, ChemicalQuadTreeDebug, ShowChemicalQuadTree},
        systems::{build_quadtree, visualise_quadtree},
    },
};

// TODO Collision and bound check in cell editor messes up the time reversal
// TODO Strange bug where cell can split even though it has split type never split (And the daughters are bigger than the parent)
// TODO Possible bug in simulation mode where cells that split don't get an energy check until the decay function is ran (May stay alive for a frame too long)
// TODO Cell editor jitter when reversing split time (Possibly need to put a marker on cells that are in the process of splitting)
// TODO Quadtree size would be wrong if simulation or cell editor dish changed size

pub mod cell_editor;
pub mod cells;
pub mod collision;
pub mod despawning;
pub mod game_mode;
pub mod genomes;
pub mod helpers;
pub mod input;
pub mod serialisation;
pub mod simulation;
pub mod spatial_partitioning;
pub mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(ShapePlugin)
        .add_plugins(Material2dPlugin::<CellMaterial>::default())
        .add_plugins(Material2dPlugin::<SelectionCellMaterial>::default())
        .add_plugins(Material2dPlugin::<ChemicalMaterial>::default())
        .init_state::<GameMode>()
        .init_resource::<GenomeCollection>()
        .init_resource::<SimulationState>()
        .init_resource::<ChemicalTimer>()
        .init_resource::<CellEditorUiStyleApplied>()
        .init_resource::<CellEditorState>()
        .init_resource::<CellQuadTree>()
        .init_resource::<ShowCellQuadTree>()
        .init_resource::<ChemicalQuadTree>()
        .init_resource::<ShowChemicalQuadTree>()
        .add_message::<CellEditorInitialGenomeMessage>()
        .add_message::<CellEditorAgeMessage>()
        .add_message::<CellEditorSelectedGenomeMessage>()
        .add_message::<CellEditorColourMessage>()
        .add_message::<CellEditorSplitAngleMessage>()
        //
        // --------------------- Mode Independent Systems ----------------------
        //
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, apply_pending_despawns.run_if(state_changed::<GameMode>)) // Need to do despawning right now when GameMode changes
        .add_systems(Update, mode_independent_keyboard_event_reader)
        .add_systems(PostUpdate, apply_pending_despawns) // Despawn after the update in most cases
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
                visualise_quadtree::<CellQuadTree, ShowCellQuadTree, CellQuadTreeDebug>,
                visualise_quadtree::<ChemicalQuadTree, ShowChemicalQuadTree, ChemicalQuadTreeDebug>,
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
                cell_editor_initial_genome_message_reader,
                cell_editor_age_message_reader,
                cell_editor_selected_genome_message_reader,
                cell_editor_colour_message_reader,
                cell_editor_split_angle_message_reader,
                remove_selection_borders,
                remove_negative_aged_cells,
                modify_cell_energy,
                remove_low_energy_cells,
                add_selection_borders,
                draw_cell_info,
                split_cells,
                reverse_splits,
                build_quadtree::<CellQuadTree, Cell>,
                collision_system,
                bound_cells,
                visualise_quadtree::<CellQuadTree, ShowCellQuadTree, CellQuadTreeDebug>,
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
