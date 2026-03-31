use bevy::prelude::*;

use crate::{
    cell_editor::{
        drawing::{SplitAngleArrow, draw_split_angle_arrow_as_child},
        state::CellEditorState,
        systems::CellTimeOfBirth,
    },
    cells::{Cell, CellMaterial},
    genomes::GenomeCollection,
};

pub const SELECTION_COLOUR: Color = Color::linear_rgb(1.0, 1.0, 0.0);
pub const SELECTION_SCALE: f32 = 1.2;

#[derive(Component)]
pub struct SelectedCell;

#[derive(Component)]
pub struct SelectionBorder;

#[derive(Message, Debug, Clone)]
pub struct CellEditorInitialGenomeMessage;

#[derive(Message, Debug, Clone)]
pub struct CellEditorAgeMessage;

#[derive(Message, Debug, Clone)]
pub struct CellEditorSelectedGenomeMessage;

#[derive(Message, Debug, Clone)]
pub struct CellEditorColourMessage;

#[derive(Message, Debug, Clone)]
pub struct CellEditorSplitAngleMessage;

// TODO
#[allow(clippy::needless_pass_by_value)]
pub fn cell_editor_initial_genome_message_reader(
    events: MessageReader<CellEditorInitialGenomeMessage>,
    // state: Res<CellEditorState>,
) {
    if !events.is_empty() {
        // Do something
    }
}

// TODO
#[allow(clippy::needless_pass_by_value)]
pub fn cell_editor_age_message_reader(
    events: MessageReader<CellEditorAgeMessage>,
    state: Res<CellEditorState>,
    mut cells: Query<(&mut Cell, Option<&CellTimeOfBirth>)>,
) {
    if !events.is_empty() {
        for (mut cell, time_of_birth) in &mut cells {
            if let Some(time_of_birth) = time_of_birth {
                cell.age = state.age - time_of_birth.0;
            } else {
                cell.age = state.age;
            }
        }
    }
}

// TODO
#[allow(clippy::needless_pass_by_value)]
pub fn cell_editor_selected_genome_message_reader(
    events: MessageReader<CellEditorSelectedGenomeMessage>,
    mut commands: Commands,
    selected_entities: Query<Entity, With<SelectedCell>>,
    state: Res<CellEditorState>,
    cells_with_entity: Query<(Entity, &Cell)>,
) {
    if !events.is_empty() {
        for entity in selected_entities {
            commands.entity(entity).remove::<SelectedCell>();
        }

        for (entity, cell) in cells_with_entity {
            if cell.genome_id == state.selected_genome {
                commands.entity(entity).insert(SelectedCell);
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn cell_editor_colour_message_reader(
    events: MessageReader<CellEditorColourMessage>,
    genome_collection: Res<GenomeCollection>,
    mut selected_materials: Query<&mut MeshMaterial2d<CellMaterial>, With<SelectedCell>>,
    state: Res<CellEditorState>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    if !events.is_empty() {
        for material in &mut selected_materials {
            if let Some(mat) = materials.get_mut(&material.0) {
                mat.colour = state.get_selected_genome(&genome_collection).colour.to_linear().to_vec4();
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn cell_editor_split_angle_message_reader(
    mut commands: Commands,
    events: MessageReader<CellEditorSplitAngleMessage>,
    genome_collection: Res<GenomeCollection>,
    state: Res<CellEditorState>,
    arrows: Query<(Entity, &ChildOf), With<SplitAngleArrow>>,
    selected_entities: Query<Entity, With<SelectedCell>>,
    selected_cells: Query<(Entity, &Cell), With<SelectedCell>>,
) {
    if !events.is_empty() {
        // Despawn the previous arrows
        for (arrow_entity, child_of) in arrows {
            if selected_entities.get(child_of.parent()).is_ok() {
                commands.entity(arrow_entity).despawn();
            }
        }

        // Spawn the arrows back in with their new angles
        for (entity, cell) in selected_cells {
            draw_split_angle_arrow_as_child(&mut commands, &genome_collection, &state, entity, cell);
        }
    }
}

pub fn add_selection_borders(
    mut commands: Commands,
    mut materials: ResMut<Assets<CellMaterial>>,
    query: Query<(Entity, &Mesh2d), Added<SelectedCell>>,
) {
    for (entity, mesh) in query.iter() {
        let border_material = materials.add(CellMaterial {
            colour: SELECTION_COLOUR.to_linear().to_vec4(),
        });

        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                mesh.clone(),
                MeshMaterial2d(border_material),
                Transform::from_xyz(0., 0., -0.1).with_scale(Vec3::splat(SELECTION_SCALE)),
                SelectionBorder,
            ));
        });
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn remove_selection_borders(
    mut commands: Commands,
    mut removed_selections: RemovedComponents<SelectedCell>,
    children_query: Query<&Children>,
    border_query: Query<(), With<SelectionBorder>>,
) {
    for entity in removed_selections.read() {
        if let Ok(children) = children_query.get(entity) {
            for &child in children {
                if border_query.get(child).is_ok() {
                    commands.entity(child).despawn();
                }
            }
        }
    }

    // // SelectedCell was changed on this frame
    // if !removed_selections.is_empty() {
    //     for entity in borders {
    //         commands.entity(entity).despawn();
    //     }
    // }
}
