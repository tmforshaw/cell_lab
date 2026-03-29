use bevy::prelude::*;

use crate::{
    cell::Cell,
    cell_editor::{events::SelectedCell, history::SplitHistory},
    cell_material::CellMaterial,
    dish::DishMarker,
    genome::{Genome, GenomeBank, GenomeId},
};

#[derive(Resource, Default)]
pub struct CellEditorState {
    pub selected_genome: GenomeId,
    pub age: f32,
    pub genomes: GenomeBank,
    pub history: SplitHistory,
}

impl CellEditorState {
    #[must_use]
    pub fn get_selected_genome(&self) -> &Genome {
        &self.genomes[self.selected_genome]
    }

    #[must_use]
    pub fn get_selected_genome_mut(&mut self) -> &mut Genome {
        &mut self.genomes[self.selected_genome]
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn init_cell_editor_mode(
    mut commands: Commands,
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
        Cell::new_bundle_with_genome(
            100.,
            state.selected_genome,
            Vec2::ZERO,
            Vec2::ZERO,
            state.genomes[state.selected_genome].colour,
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
