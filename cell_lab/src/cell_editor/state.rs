use bevy::prelude::*;

use crate::{
    cell_editor::{editor_age::CellEditorAge, events::SelectedCell, history::SplitHistory},
    cells::{CELL_STARTING_ENERGY, Cell, CellMaterial},
    genomes::{Genome, GenomeBank, GenomeBankId, GenomeCollection, GenomeId},
    simulation::dish::{Dish, DishMarker},
};

const CELL_EDITOR_SIZE: Vec2 = Vec2::new(1200., 1200.);
const CELL_EDITOR_CELL_SIZE_PER_MASS: f32 = 50.;

#[derive(Resource)]
pub struct CellEditorState {
    pub selected_genome_bank: GenomeBankId,
    pub selected_genome: GenomeId,
    pub editor_age: CellEditorAge,
    pub dish: Dish,
    pub history: SplitHistory,
    pub cell_size_per_mass: f32,
    // pub energy_gain_rate:
}

impl Default for CellEditorState {
    fn default() -> Self {
        Self {
            selected_genome_bank: GenomeBankId::default(),
            selected_genome: GenomeId::default(),
            editor_age: CellEditorAge::default(),
            dish: Dish::new(CELL_EDITOR_SIZE),
            history: SplitHistory::default(),
            cell_size_per_mass: CELL_EDITOR_CELL_SIZE_PER_MASS,
        }
    }
}

impl CellEditorState {
    #[must_use]
    pub fn get_selected_genome<'a>(&self, genome_collection: &'a GenomeCollection) -> &'a Genome {
        &genome_collection[self.selected_genome_bank][self.selected_genome]
    }

    #[must_use]
    pub fn get_selected_genome_mut<'a>(&mut self, genome_collection: &'a mut GenomeCollection) -> &'a mut Genome {
        &mut genome_collection[self.selected_genome_bank][self.selected_genome]
    }

    #[must_use]
    pub fn get_selected_genome_bank<'a>(&self, genome_collection: &'a GenomeCollection) -> &'a GenomeBank {
        &genome_collection[self.selected_genome_bank]
    }

    #[must_use]
    pub fn get_selected_genome_bank_mut<'a>(&mut self, genome_collection: &'a mut GenomeCollection) -> &'a mut GenomeBank {
        &mut genome_collection[self.selected_genome_bank]
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn init_cell_editor_mode(
    mut commands: Commands,
    genome_collection: Res<GenomeCollection>,
    mut state: ResMut<CellEditorState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    // TODO Maybe don't need this
    // Reset the simulation age
    state.editor_age = CellEditorAge::default();

    // Spawn dish
    commands.spawn(state.dish.into_bundle());

    // Create a bundle for the selected genome, make it selected, then spawn it
    commands.spawn((
        Cell::new_bundle(
            CELL_STARTING_ENERGY,
            state.selected_genome,
            state.selected_genome_bank,
            state.cell_size_per_mass,
            Vec2::ZERO,
            Vec2::ZERO,
            &genome_collection,
            &mut meshes,
            &mut materials,
        ),
        SelectedCell,
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
