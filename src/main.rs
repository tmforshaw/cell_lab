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

use crate::{
    cell::{bound_cells, cell_decay, cells_absorb_chemical, cells_do_meiosis, increment_cell_age, move_cells},
    cell_editor::{CellEditorState, exit_cell_editor_mode, init_cell_editor_mode, split_cells},
    cell_editor_events::{
        CellEditorAgeMessage, CellEditorColourMessage, CellEditorSelectedGenomeMessage, add_selection_borders,
        cell_editor_age_message_reader, cell_editor_colour_message_reader, cell_editor_selected_genome_message_reader,
        remove_selection_borders,
    },
    cell_editor_ui::{CellEditorUiStyleApplied, cell_editor_ui_update},
    cell_material::CellMaterial,
    chemical::{ChemicalMaterial, ChemicalTimer, spawn_chemicals},
    input::{cell_editor_mode_keyboard_event_reader, play_mode_keyboard_event_reader},
    state::{GameMode, PlayModeState, exit_play_mode, init_play_mode},
};

// TODO use genomes when setting material colour

pub mod cell;
pub mod cell_editor;
pub mod cell_editor_events;
pub mod cell_editor_ui;
pub mod cell_material;
pub mod chemical;
pub mod dish;
pub mod genome;
pub mod helpers;
pub mod input;
pub mod state;
pub mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(Material2dPlugin::<CellMaterial>::default())
        .add_plugins(Material2dPlugin::<ChemicalMaterial>::default())
        // .init_state::<GameMode>()
        .insert_state(GameMode::CellEditor)
        .init_resource::<PlayModeState>()
        .init_resource::<ChemicalTimer>()
        .init_resource::<CellEditorUiStyleApplied>()
        .init_resource::<CellEditorState>()
        .add_systems(Startup, setup)
        .add_message::<CellEditorAgeMessage>()
        .add_message::<CellEditorSelectedGenomeMessage>()
        .add_message::<CellEditorColourMessage>()
        //
        // ---------------------------- Play Mode -----------------------------
        //
        .add_systems(OnEnter(GameMode::Play), init_play_mode)
        .add_systems(
            Update,
            (
                play_mode_keyboard_event_reader,
                increment_cell_age,
                spawn_chemicals,
                move_cells,
                bound_cells,
                cells_absorb_chemical,
                cells_do_meiosis,
            )
                .run_if(in_state(GameMode::Play)),
        )
        .add_systems(PostUpdate, (cell_decay).run_if(in_state(GameMode::Play)))
        .add_systems(OnExit(GameMode::Play), exit_play_mode)
        //
        // ------------------------- Cell Editor Mode --------------------------
        //
        .add_systems(OnEnter(GameMode::CellEditor), init_cell_editor_mode)
        .add_systems(
            Update,
            (
                cell_editor_mode_keyboard_event_reader,
                cell_editor_age_message_reader,
                cell_editor_selected_genome_message_reader,
                cell_editor_colour_message_reader,
                remove_selection_borders,
                add_selection_borders,
                split_cells,
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
        .run();
}

// Spawn cells and chemicals
fn setup(mut commands: Commands) {
    // 2D camera
    commands.spawn(Camera2d);
}
