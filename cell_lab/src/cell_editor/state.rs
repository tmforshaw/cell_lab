use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    cell_editor::{
        editor_age::CellEditorAge,
        snapshot::{CellEditorSimulationState, CellHistoryCache},
        ui_dialog::CellEditorUiDialogState,
    },
    cells::Cell,
    genomes::{Genome, GenomeBank, GenomeId, GenomeMode, GenomeModeId},
    simulation::dish::{Dish, DishMarker},
};

pub const CELL_EDITOR_SIZE: Vec2 = WINDOW_SIZE;
pub const CELL_EDITOR_CELL_SIZE_PER_MASS: f32 = 50.;
const CELL_EDITOR_CELL_ENERGY_GAIN_RATE: f32 = 2.;
const CELL_EDITOR_CELL_ENERGY_DECAY_RATE: f32 = 0.5;
const CELL_EDITOR_SIMULATION_DELTA_TIME: f32 = 0.02;

#[derive(Resource)]
pub struct CellEditorState {
    pub selected_genome_mode: GenomeModeId,
    pub selected_genome: GenomeId,
    pub editor_age: CellEditorAge,
    pub dish: Dish,
    pub cell_size_per_mass: f32,
    pub cell_energy_gain_rate: f32,
    pub cell_energy_decay_rate: f32,
    pub dialogs: CellEditorUiDialogState,
    pub simulation_delta_time: f32,
}

impl Default for CellEditorState {
    fn default() -> Self {
        Self {
            selected_genome_mode: GenomeModeId::default(),
            selected_genome: GenomeId::default(),
            editor_age: CellEditorAge::default(),
            dish: Dish::new(CELL_EDITOR_SIZE),
            cell_size_per_mass: CELL_EDITOR_CELL_SIZE_PER_MASS,
            cell_energy_gain_rate: CELL_EDITOR_CELL_ENERGY_GAIN_RATE,
            cell_energy_decay_rate: CELL_EDITOR_CELL_ENERGY_DECAY_RATE,
            dialogs: CellEditorUiDialogState::default(),
            simulation_delta_time: CELL_EDITOR_SIMULATION_DELTA_TIME,
        }
    }
}

impl CellEditorState {
    #[must_use]
    pub fn get_selected_genome_mode<'a>(&self, genome_bank: &'a GenomeBank) -> &'a GenomeMode {
        &genome_bank[self.selected_genome][self.selected_genome_mode]
    }

    #[must_use]
    pub fn get_selected_genome_mode_mut<'a>(&mut self, genome_bank: &'a mut GenomeBank) -> &'a mut GenomeMode {
        &mut genome_bank[self.selected_genome][self.selected_genome_mode]
    }

    #[must_use]
    pub fn get_selected_genome<'a>(&self, genome_bank: &'a GenomeBank) -> &'a Genome {
        &genome_bank[self.selected_genome]
    }

    #[must_use]
    pub fn get_selected_genome_mut<'a>(&mut self, genome_bank: &'a mut GenomeBank) -> &'a mut Genome {
        &mut genome_bank[self.selected_genome]
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn init_cell_editor_mode(mut commands: Commands, state: ResMut<CellEditorState>) {
    // Insert the simulation resources
    commands.insert_resource(CellHistoryCache::default());
    commands.insert_resource(CellEditorSimulationState::default());

    // Spawn dish
    commands.spawn(state.dish.into_bundle());
}

pub fn exit_cell_editor_mode(
    mut commands: Commands,
    dishes: Query<Entity, With<DishMarker>>,
    cells: Query<Entity, With<Cell>>,
    mut state: ResMut<CellEditorState>,
) {
    // Remove the simulation resources
    commands.remove_resource::<CellHistoryCache>();
    commands.remove_resource::<CellEditorSimulationState>();

    // Close all dialogs
    state.dialogs.close_all_dialogs();

    for entity in dishes {
        commands.entity(entity).despawn();
    }

    for entity in cells {
        commands.entity(entity).despawn();
    }
}
