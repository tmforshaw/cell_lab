use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    cell_editor::state::CellEditorState,
    cells::{Cell, CellMaterial, SelectionCellMaterial},
    despawning::PendingDespawn,
    game::game_parameters::GameParameters,
    genomes::GenomeBank,
};

#[derive(Component)]
pub struct SelectedCell;

#[derive(Component)]
pub struct SelectionBorder;

#[derive(Message, Debug, Clone)]
pub struct CellEditorInitialGenomeModeMessage;

#[derive(Message, Debug, Clone)]
pub struct CellEditorSelectedGenomeModeMessage;

#[derive(Message, Debug, Clone)]
pub struct CellEditorColourMessage;

#[allow(clippy::needless_pass_by_value)]
pub fn cell_editor_initial_genome_mode_message_reader(events: MessageReader<CellEditorInitialGenomeModeMessage>) {
    if !events.is_empty() {
        // Do something
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn cell_editor_selected_genome_mode_message_reader(
    events: MessageReader<CellEditorSelectedGenomeModeMessage>,
    mut commands: Commands,
    selected_entities: Query<Entity, (With<SelectedCell>, Without<PendingDespawn>)>,
    state: Res<CellEditorState>,
    cells_with_entity: Query<(Entity, &Cell), Without<PendingDespawn>>,
) {
    if !events.is_empty() {
        for entity in selected_entities {
            commands.entity(entity).remove::<SelectedCell>();
        }

        for (entity, cell) in cells_with_entity {
            if cell.genome_mode_id == state.selected_genome_mode {
                commands.entity(entity).insert(SelectedCell);
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn cell_editor_colour_message_reader(
    events: MessageReader<CellEditorColourMessage>,
    genome_bank: Res<GenomeBank>,
    mut selected_materials: Query<&mut MeshMaterial2d<CellMaterial>, (With<SelectedCell>, Without<PendingDespawn>)>,
    state: Res<CellEditorState>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    if !events.is_empty() {
        for material in &mut selected_materials {
            if let Some(mat) = materials.get_mut(&material.0) {
                mat.colour = state.get_selected_genome_mode(&genome_bank).colour.to_linear().to_vec4();
            }
        }
    }
}

#[allow(clippy::type_complexity, clippy::needless_pass_by_value)]
pub fn remove_selection_borders(
    mut commands: Commands,
    state: Res<CellEditorState>,
    selection: Query<(Entity, &ChildOf), (With<SelectionBorder>, Without<PendingDespawn>)>,
    selected: Query<(Entity, &Cell), (With<SelectedCell>, Without<PendingDespawn>)>,
    mut removed: RemovedComponents<SelectedCell>,
    selection_border_cell: Query<&Cell, (Without<SelectedCell>, Without<PendingDespawn>)>,
) {
    // Create HashMap of both the selected entities, and the entities whose SelectedCell Component was removed this frame
    let selected_or_removed = selected
        .iter()
        .chain(
            removed
                .read()
                .filter_map(|entity| selection_border_cell.get(entity).ok().map(|cell| (entity, cell))),
        )
        .collect::<HashMap<_, _>>();

    // Combine selection borders with
    for (entity, parent) in selection {
        // If the parent's genome mode id is not the selected genome mode
        if let Some(&parent_cell) = selected_or_removed.get(&parent.parent())
            && parent_cell.genome_mode_id != state.selected_genome_mode
        {
            // Remove SelectedCell Marker From Parent
            commands.entity(parent.parent()).remove::<SelectedCell>();

            // Despawn Selection Entity
            commands.entity(entity).insert(PendingDespawn);
        }
    }
}

#[allow(clippy::type_complexity, clippy::needless_pass_by_value)]
pub fn add_selection_borders(
    mut commands: Commands,
    state: Res<CellEditorState>,
    param: Res<GameParameters>,
    mut selection_materials: ResMut<Assets<SelectionCellMaterial>>,
    mut cell_materials: ResMut<Assets<CellMaterial>>,
    added: Query<(Entity, &Cell, &Mesh2d, &MeshMaterial2d<CellMaterial>), (Added<SelectedCell>, Without<PendingDespawn>)>,
    unselected: Query<(Entity, &Cell, &Mesh2d, &MeshMaterial2d<CellMaterial>), (Without<SelectedCell>, Without<PendingDespawn>)>,
) {
    // Add markers to unselected cells that need to be selected
    for (entity, cell, _mesh, cell_material) in unselected {
        // This cell should be selected
        if cell.genome_mode_id == state.selected_genome_mode {
            // Add SelectedCell Marker
            commands.entity(entity).insert(SelectedCell);

            // Set the show_cell_info flag of CellMaterial to true
            if let Some(material) = cell_materials.get_mut(cell_material.id()) {
                material.show_cell_info = true as u32;
            }
        }
    }

    // Add selection to unselected (or recently selected) cells if necessary
    for (entity, _cell, cell_mesh, _cell_material) in unselected
        .iter()
        // Only add selection mesh to cells who are the selected genome mode
        .filter(|(_, cell, _, _)| cell.genome_mode_id == state.selected_genome_mode)
        .chain(added)
    {
        // Add Selection Mesh
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                cell_mesh.clone(),
                MeshMaterial2d(selection_materials.add(SelectionCellMaterial {
                    colour: param.selection_parameters.colour.to_linear().to_vec4(),
                })),
                Transform::from_xyz(0., 0., -0.1).with_scale(Vec3::splat(param.selection_parameters.scale)),
                SelectionBorder,
            ));
        });
    }
}
