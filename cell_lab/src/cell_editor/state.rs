use bevy::prelude::*;

use crate::{
    cell_editor::{
        editor_age::CellEditorAge,
        simulation::CellEditorSimulationStatus,
        snapshot::{CellEditorSimulationState, CellHistoryCache},
        ui_dialog::CellEditorUiDialogState,
    },
    cells::Cell,
    game::game_parameters::GameParameters,
    genomes::{Genome, GenomeBank, GenomeId, GenomeMode, GenomeModeId},
    simulation::dish::DishMarker,
};

#[derive(Resource, Default)]
pub struct CellEditorState {
    pub selected_genome_mode: GenomeModeId,
    pub selected_genome: GenomeId,
    pub editor_age: CellEditorAge,
    pub dialogs: CellEditorUiDialogState,
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
pub fn init_cell_editor_mode(mut commands: Commands, param: Res<GameParameters>) {
    // Insert the simulation resources
    commands.insert_resource(CellHistoryCache::default());
    commands.insert_resource(CellEditorSimulationState::default());

    // Set the simulation to NeedsRecompute
    commands.set_state(CellEditorSimulationStatus::NeedsRecompute);

    // Spawn dish
    commands.spawn(param.cell_editor_mode.dish_parameters.get_dish_bundle());
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
