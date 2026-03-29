use bevy::prelude::*;

use crate::{
    cell::Cell,
    cell_editor_events::SelectedCell,
    cell_material::CellMaterial,
    dish::DishMarker,
    genome::{CellSplitType, Genome, GenomeBank, GenomeId, get_daughter_data},
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

#[derive(Component)]
pub struct CellTimeOfBirth(pub f32);

#[allow(clippy::needless_pass_by_value)]
pub fn split_cells(
    mut commands: Commands,
    state: Res<CellEditorState>,
    cells: Query<(Entity, &Cell, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    for (entity, cell, transform) in cells {
        match state.genomes[cell.genome_id].split_type {
            CellSplitType::Age => {
                let cell_genome = &state.genomes[cell.genome_id];

                // Cell is ready to split
                if cell.age >= cell_genome.split_age {
                    let (d1, d2) = get_daughter_data(
                        cell,
                        cell.genome_id,
                        transform.translation.xy(),
                        transform.scale.xy(),
                        &state.genomes,
                    );

                    // Set the first daughter's parameters, and get its bundle
                    let d1_bundle = (
                        Cell::new_bundle_with_genome(
                            d1.energy,
                            d1.genome_id,
                            d1.velocity,
                            d1.position,
                            d1.colour,
                            &mut meshes,
                            &mut materials,
                        ),
                        CellTimeOfBirth(state.age),
                    );

                    // Spawn the daughter with the selected cell marker, if necessary
                    if state.selected_genome == d1.genome_id {
                        commands.spawn((d1_bundle, SelectedCell));
                    } else {
                        commands.spawn(d1_bundle);
                    }

                    // Set the second daughter's parameters, and get its bundle
                    let d2_bundle = (
                        Cell::new_bundle_with_genome(
                            d2.energy,
                            d2.genome_id,
                            d2.velocity,
                            d2.position,
                            d2.colour,
                            &mut meshes,
                            &mut materials,
                        ),
                        CellTimeOfBirth(state.age),
                    );

                    // Spawn the daughter with the selected cell marker, if necessary
                    if state.selected_genome == d2.genome_id {
                        commands.spawn((d2_bundle, SelectedCell));
                    } else {
                        commands.spawn(d2_bundle);
                    }

                    // Despawn the parent cell
                    commands.entity(entity).despawn();
                }
            }
            CellSplitType::Energy => todo!(),
            CellSplitType::Never => {}
        }
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

    // Spawn the genome bank's initial cell, and make it selected
    commands.spawn((
        Cell::new_bundle_with_genome(
            100.,
            state.genomes.initial,
            Vec2::ZERO,
            Vec2::ZERO,
            state.genomes[state.genomes.initial].colour,
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

// ---------------------------------------------------------------------
