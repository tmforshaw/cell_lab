use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    cell::{Cell, Velocity},
    cell_editor_events::SelectedCell,
    cell_material::CellMaterial,
    dish::DishMarker,
    genome::{CellSplitType, Genome, GenomeBank, GenomeId},
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
    mut cells: Query<(Entity, &mut Cell, &mut Transform, &mut Velocity, Option<&mut CellTimeOfBirth>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    for (entity, mut cell, mut transform, mut velocity, time_of_birth) in &mut cells {
        match state.genomes[cell.genome_id].split_type {
            CellSplitType::Age => {
                let cell_genome = &state.genomes[cell.genome_id];

                // Cell is ready to split
                if cell.age >= cell_genome.split_age {
                    // Split energy depending on split fraction
                    let d1_energy = cell.energy * cell_genome.split_fraction;
                    let d2_energy = cell.energy - d1_energy;

                    let d1_genome_id = cell_genome.daughter_genomes.0;
                    let d2_genome_id = cell_genome.daughter_genomes.1;

                    let d1_colour = state.genomes[d1_genome_id].colour;
                    let d2_colour = state.genomes[d2_genome_id].colour;

                    // Give velocity depending on split angle
                    let velocity_mag = cell_genome.split_force;
                    let d1_velocity = velocity_mag * Vec2::Y.rotate(Vec2::from_angle(cell_genome.split_angle - PI / 2.));
                    let d2_velocity = velocity_mag * Vec2::Y.rotate(Vec2::from_angle(cell_genome.split_angle + PI / 2.));

                    // Offset the daughters
                    let d1_position = (transform.scale.xy() * cell_genome.split_fraction * d1_velocity.normalize()).extend(0.);
                    let d2_position =
                        (transform.scale.xy() * (1. - cell_genome.split_fraction) * d2_velocity.normalize()).extend(0.);

                    // If one of the daughter cells has the same genome as first daughter, repurpose it
                    if cell.genome_id == d1_genome_id {
                        // Spawn new cell based on other daughter
                        let d2_bundle = Cell::new_bundle_with_genome(
                            d2_energy,
                            d2_genome_id,
                            d2_velocity,
                            transform.translation.xy() + d2_position.xy(),
                            d2_colour,
                            &mut meshes,
                            &mut materials,
                        );

                        // Spawn the daughter with the selected cell marker, if necessary
                        if state.selected_genome == d2_genome_id {
                            commands.spawn((d2_bundle, CellTimeOfBirth(state.age), SelectedCell));
                        } else {
                            commands.spawn((d2_bundle, CellTimeOfBirth(state.age)));
                        }

                        // First daughter is the same as this cell
                        cell.age = 0.;
                        cell.energy = d1_energy;
                        transform.scale = cell.get_size().extend(0.);
                        transform.translation += d1_position;
                        *velocity = Velocity(d1_velocity);
                    } else {
                        // Spawn new cell based on other daughter
                        let d1_bundle = Cell::new_bundle_with_genome(
                            d1_energy,
                            d1_genome_id,
                            d1_velocity,
                            transform.translation.xy() + d1_position.xy(),
                            d1_colour,
                            &mut meshes,
                            &mut materials,
                        );

                        // Spawn the daughter with the selected cell marker, if necessary
                        if state.selected_genome == d1_genome_id {
                            commands.spawn((d1_bundle, CellTimeOfBirth(state.age), SelectedCell));
                        } else {
                            commands.spawn((d1_bundle, CellTimeOfBirth(state.age)));
                        }

                        // Second daughter is the same as this cell (or neither daughter is the same as original cell)
                        cell.age = 0.;
                        cell.energy = d2_energy;
                        transform.scale = cell.get_size().extend(0.);
                        transform.translation += d2_position;
                        *velocity = Velocity(d2_velocity);
                        cell.genome_id = d2_genome_id; // Set the genome incase neither daughter was the same
                    }

                    // If repurposed cell already has a CellTimeOfBirth, update it
                    if let Some(mut time_of_birth) = time_of_birth {
                        time_of_birth.0 = state.age;
                    } else {
                        // Otherwise insert a new CellTimeOfBirth component
                        commands.entity(entity).insert(CellTimeOfBirth(state.age));
                    }
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
