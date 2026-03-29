use bevy::prelude::*;

use crate::{cell::Cell, cell_editor::CellEditorState, cell_material::CellMaterial};

pub const SELECTION_COLOUR: Color = Color::linear_rgb(1.0, 1.0, 0.0);
pub const SELECTION_SCALE: f32 = 1.2;

#[derive(Component)]
pub struct SelectedCell;

#[derive(Component)]
pub struct SelectionBorder;

#[derive(Debug, Copy, Clone)]
pub enum CellEditorParameter {
    Age,
    SelectedGenome,
    CellType,
    Daughter1Mode,
    Daughter2Mode,
    Colour,
    SplitFraction,
    SplitThreshold,
}

#[derive(Message, Debug, Clone)]
pub struct CellEditorMessage {
    pub param: CellEditorParameter,
}

#[allow(clippy::needless_pass_by_value)]
pub fn cell_editor_message_reader(
    mut events: MessageReader<CellEditorMessage>,
    mut commands: Commands,
    selected_cells: Query<Entity, With<SelectedCell>>,
    state: Res<CellEditorState>,
    cells: Query<(Entity, &Cell)>,
) {
    for ev in events.read() {
        println!("Event: {ev:?}");

        match ev.param {
            CellEditorParameter::Age => todo!(),
            CellEditorParameter::SelectedGenome => {
                for entity in selected_cells {
                    commands.entity(entity).remove::<SelectedCell>();
                }

                for (entity, cell) in cells {
                    if cell.genome.id == state.get_selected_genome().id {
                        commands.entity(entity).insert(SelectedCell);
                    }
                }
            }
            CellEditorParameter::CellType => todo!(),
            CellEditorParameter::Daughter1Mode => todo!(),
            CellEditorParameter::Daughter2Mode => todo!(),
            CellEditorParameter::Colour => todo!(),
            CellEditorParameter::SplitFraction => todo!(),
            CellEditorParameter::SplitThreshold => todo!(),
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
