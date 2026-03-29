use bevy::prelude::*;

use crate::{
    cell::{Cell, Velocity},
    cell_editor::{events::SelectedCell, history::SplitHistoryData, state::CellEditorState},
    cell_material::CellMaterial,
    genome::{CellSplitType, get_daughter_data},
};

#[derive(Component, Debug, Clone)]
pub struct CellTimeOfBirth(pub f32);

#[allow(clippy::needless_pass_by_value)]
pub fn split_cells(
    mut commands: Commands,
    mut state: ResMut<CellEditorState>,
    cells: Query<(Entity, &Cell, &Transform, &Velocity, Option<&CellTimeOfBirth>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    for (entity, parent, transform, parent_velocity, parent_time_of_birth) in cells {
        match state.genomes[parent.genome_id].split_type {
            CellSplitType::Age => {
                let parent_genome = &state.genomes[parent.genome_id];

                // Parent is ready to split
                if parent.age >= parent_genome.split_age {
                    let (d1, d2) = get_daughter_data(
                        parent,
                        parent.genome_id,
                        transform.translation.xy(),
                        transform.scale.xy(),
                        &state.genomes,
                    );

                    let time_of_birth = if let Some(&CellTimeOfBirth(parent_time_of_birth)) = parent_time_of_birth {
                        CellTimeOfBirth(parent_time_of_birth + state.genomes[parent.genome_id].split_age)
                    } else {
                        CellTimeOfBirth(state.genomes[parent.genome_id].split_age)
                    };

                    let daughter_age = (state.age - time_of_birth.0).min(0.);

                    // Set the first daughter's parameters, and get its bundle
                    let d1_bundle = (
                        Cell::new_bundle_with_genome_and_age(
                            d1.energy,
                            daughter_age,
                            d1.genome_id,
                            d1.velocity,
                            d1.position,
                            d1.colour,
                            &mut meshes,
                            &mut materials,
                        ),
                        time_of_birth.clone(),
                    );

                    // Set the second daughter's parameters, and get its bundle
                    let d2_bundle = (
                        Cell::new_bundle_with_genome_and_age(
                            d2.energy,
                            daughter_age,
                            d2.genome_id,
                            d2.velocity,
                            d2.position,
                            d2.colour,
                            &mut meshes,
                            &mut materials,
                        ),
                        time_of_birth.clone(),
                    );

                    // Spawn the daughter with the selected cell marker, if necessary
                    if state.selected_genome == d1.genome_id {
                        commands.spawn((d1_bundle, SelectedCell));
                    } else {
                        commands.spawn(d1_bundle);
                    }

                    // Spawn the daughter with the selected cell marker, if necessary
                    if state.selected_genome == d2.genome_id {
                        commands.spawn((d2_bundle, SelectedCell));
                    } else {
                        commands.spawn(d2_bundle);
                    }

                    // Despawn the parent cell
                    commands.entity(entity).despawn();

                    // Add this split to the split history
                    let simulation_age = state.age;
                    state.history.insert(SplitHistoryData {
                        simulation_age,
                        parent: parent.clone(),
                        parent_genome_id: parent.genome_id,
                        parent_position: transform.translation.xy(),
                        parent_velocity: parent_velocity.0,
                        parent_time_of_birth: parent_time_of_birth.map_or(CellTimeOfBirth(0.0), std::clone::Clone::clone),
                    });
                }
            }
            CellSplitType::Energy => todo!(),
            CellSplitType::Never => {}
        }
    }
}

pub fn reverse_splits(
    mut commands: Commands,
    mut state: ResMut<CellEditorState>,
    cells: Query<(Entity, &CellTimeOfBirth)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    while let Some(current) = state.history.get() {
        // If the age hasn't went back enough to reach any split events
        if state.age >= current.simulation_age {
            break;
        }

        // Despawn any daughters which have age that is too low
        for (entity, CellTimeOfBirth(birth)) in cells {
            if *birth >= current.simulation_age {
                commands.entity(entity).despawn();
            }
        }

        println!("{:?}", current);

        // Restore the parent
        let bundle = Cell::new_bundle_with_genome_and_age(
            current.parent.energy,
            current.parent.age,
            current.parent.genome_id,
            current.parent_velocity,
            current.parent_position,
            state.genomes[current.parent.genome_id].colour,
            &mut meshes,
            &mut materials,
        );

        // Spawn the bundle, selecting if necessary
        let time_of_birth = current.parent_time_of_birth;
        if state.selected_genome == current.parent.genome_id {
            commands.spawn((bundle, time_of_birth, SelectedCell));
        } else {
            commands.spawn((bundle, time_of_birth));
        }

        // Move the history back
        state.history.decrement_current();
    }
}
