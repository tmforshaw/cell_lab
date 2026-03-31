use bevy::prelude::*;

use crate::{
    cell::Cell,
    cell_editor::{events::SelectedCell, history::SplitHistory},
    cell_material::CellMaterial,
    dish::DishMarker,
    genome::{Genome, GenomeId},
    genome_bank::{GenomeBank, GenomeBankId, GenomeCollection},
};

#[derive(Resource, Default)]
pub struct CellEditorState {
    pub selected_genome_bank: GenomeBankId,
    pub selected_genome: GenomeId,
    pub age: f32,
    pub history: SplitHistory,
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
    // Reset the simulation age
    state.age = 0.;

    // Spawn bacground
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.2, 0.2, 0.2),
            custom_size: Some(Vec2::splat(1200.)),
            ..default()
        },
        Transform::from_xyz(0., 0., 0.),
        DishMarker,
    ));

    // Create a bundle for the selected genome, make it selected, then spawn it
    commands.spawn((
        Cell::new_bundle(
            100.,
            state.selected_genome,
            state.selected_genome_bank,
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
