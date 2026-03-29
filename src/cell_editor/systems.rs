use bevy::prelude::*;

use crate::{
    cell::Cell,
    cell_editor::{events::SelectedCell, state::CellEditorState},
    cell_material::CellMaterial,
    genome::{CellSplitType, get_daughter_data},
};

#[derive(Component, Clone)]
pub struct CellTimeOfBirth(pub f32);

#[allow(clippy::needless_pass_by_value)]
pub fn split_cells(
    mut commands: Commands,
    state: Res<CellEditorState>,
    cells: Query<(Entity, &Cell, &Transform, Option<&CellTimeOfBirth>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    for (entity, parent, transform, parent_time_of_birth) in cells {
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

                    let daughter_age = (state.age - time_of_birth.0).max(0.);

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
                        time_of_birth,
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
                }
            }
            CellSplitType::Energy => todo!(),
            CellSplitType::Never => {}
        }
    }
}
