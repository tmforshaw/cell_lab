use bevy::prelude::*;

use crate::{
    cell_editor::{events::SelectedCell, history::SplitHistoryData, state::CellEditorState},
    cells::{CELL_MIN_ENERGY, Cell, CellMaterial, Velocity},
    despawning::PendingDespawn,
    genomes::GenomeCollection,
};

#[derive(Component, Debug, Clone)]
pub struct CellTimeOfBirth(pub f32);

#[allow(clippy::needless_pass_by_value)]
pub fn split_cells(
    mut commands: Commands,
    genome_collection: Res<GenomeCollection>,
    mut state: ResMut<CellEditorState>,
    cells: Query<(Entity, &Cell, &Transform, &Velocity, Option<&CellTimeOfBirth>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    // Only split cells when age is increasing
    if state.editor_age.is_decreasing() {
        return;
    }

    for (entity, parent, transform, parent_velocity, parent_time_of_birth) in cells {
        let parent_genome = parent.get_genome(&genome_collection);

        // Calculate the time of birth for these daughters
        let time_of_birth = if let Some(&CellTimeOfBirth(parent_time_of_birth)) = parent_time_of_birth {
            CellTimeOfBirth(parent_time_of_birth + parent_genome.split_age)
        } else {
            CellTimeOfBirth(parent_genome.split_age)
        };

        // Calculate the age based on when the daughters were found to be born
        let daughter_age = (state.editor_age.get_age() - time_of_birth.0).max(0.);

        // Get the bundles for the daughter's based on the parent
        if let Some((d1_bundle, d2_bundle)) = parent.split_into_daughter_bundles_with_age(
            daughter_age,
            &genome_collection,
            transform,
            parent_velocity,
            &mut meshes,
            &mut materials,
        ) {
            // Add the daughter's time of birth as a component
            let d1_bundle = (d1_bundle, time_of_birth.clone());
            let d2_bundle = (d2_bundle, time_of_birth.clone());

            // Spawn the daughter with the selected cell marker, if necessary
            if state.selected_genome == parent_genome.daughter_genomes.0 {
                commands.spawn((d1_bundle, SelectedCell));
            } else {
                commands.spawn(d1_bundle);
            }

            // Spawn the daughter with the selected cell marker, if necessary
            if state.selected_genome == parent_genome.daughter_genomes.1 {
                commands.spawn((d2_bundle, SelectedCell));
            } else {
                commands.spawn(d2_bundle);
            }

            // Despawn the parent cell
            commands.entity(entity).insert(PendingDespawn);

            // Add this split to the split history
            let editor_age = state.editor_age;
            state.history.insert(SplitHistoryData {
                editor_age,
                parent: parent.clone(),
                parent_position: transform.translation.xy(),
                parent_velocity: parent_velocity.0,
                parent_time_of_birth: parent_time_of_birth.map_or(CellTimeOfBirth(0.0), std::clone::Clone::clone),
            });
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn reverse_splits(
    mut commands: Commands,
    genome_collection: Res<GenomeCollection>,
    mut state: ResMut<CellEditorState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    // Only reverse splits when the editor age is decreasing
    if state.editor_age.is_increasing() {
        return;
    }

    while let Some(current) = state.history.get() {
        // If the age hasn't went back enough to reach any split events
        if state.editor_age.get_age() >= current.editor_age.get_age() {
            break;
        }

        // Restore the parent
        let bundle = Cell::new_bundle_with_age(
            current.parent.energy,
            current.parent.age,
            current.parent.genome_id,
            current.parent.genome_bank_id,
            state.cell_size_per_mass,
            current.parent_velocity,
            current.parent_position,
            &genome_collection,
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

#[allow(clippy::needless_pass_by_value)]
pub fn remove_negative_aged_cells(
    mut commands: Commands,
    state: ResMut<CellEditorState>,
    cells: Query<(Entity, &CellTimeOfBirth), Without<PendingDespawn>>,
) {
    // Despawn any daughters which have age that is below the current simulation age
    for (entity, CellTimeOfBirth(birth)) in cells {
        if *birth > state.editor_age.get_age() {
            commands.entity(entity).insert(PendingDespawn);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn remove_low_energy_cells(mut commands: Commands, cells: Query<(Entity, &Cell), Without<PendingDespawn>>) {
    // Despawn cells with energy below minimum allowable energy
    for (entity, cell) in cells {
        if cell.energy <= CELL_MIN_ENERGY {
            commands.entity(entity).insert(PendingDespawn);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn modify_cell_energy(state: Res<CellEditorState>, mut cells: Query<(&mut Cell, &mut Transform), Without<PendingDespawn>>) {
    // Modify the cell's energy based on age.delta(), then its size
    for (mut cell, mut transform) in &mut cells {
        // Increment or decrement energy
        cell.energy += state.cell_energy_gain_rate * state.editor_age.delta();

        // Resize the cell
        transform.scale = cell.get_size().extend(1.);
    }
}
