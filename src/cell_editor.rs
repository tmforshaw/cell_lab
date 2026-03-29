use bevy::prelude::*;

use crate::{
    cell::Cell,
    cell_editor_events::SelectedCell,
    cell_material::CellMaterial,
    dish::DishMarker,
    genome::{Genome, GenomeBank, GenomeId},
};

#[derive(Resource, Default)]
pub struct CellEditorState {
    pub selected_genome: GenomeId,
    pub age: f32,
    pub genomes: GenomeBank,
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

// ------------------------- Cell Editor Mode --------------------------

#[allow(clippy::needless_pass_by_value)]
pub fn init_cell_editor_mode(
    mut commands: Commands,
    state: Res<CellEditorState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
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

    // Spawn a default cell
    let cell_bundle = Cell::new_bundle(
        100.,
        Vec2::ZERO,
        Vec2::ZERO,
        Color::linear_rgb(0.5, 1.0, 0.5),
        &mut meshes,
        &mut materials,
    );

    // Make the cell selected if M1 is the selected genome
    if state.selected_genome == GenomeId::M1 {
        commands.spawn((cell_bundle, SelectedCell));
    } else {
        commands.spawn(cell_bundle);
    }
}

pub fn exit_cell_editor_mode(mut commands: Commands, dishes: Query<Entity, With<DishMarker>>, cells: Query<Entity, With<Cell>>) {
    for entity in dishes {
        commands.entity(entity).despawn();
    }

    for entity in cells {
        commands.entity(entity).despawn();
    }
}

// ---------------------------------------------------------------------
