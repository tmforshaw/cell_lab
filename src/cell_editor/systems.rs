use bevy::prelude::*;

use crate::{
    cell::Cell,
    cell_editor::{events::SelectedCell, state::CellEditorState},
    cell_material::CellMaterial,
    genome::{CellSplitType, get_daughter_data},
};

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
