use bevy::prelude::*;

use crate::{cell::Cell, cell_material::CellMaterial, dish::DishMarker};

#[derive(Resource, Default)]
pub struct CellEditorState {
    selected_cell: Option<Entity>,
    brush_type: BrushType,
}

#[derive(Resource, Default)]
pub enum BrushType {
    #[default]
    Select,
    Place,
    Remove,
}

// ------------------------- Cell Editor Mode --------------------------

pub fn init_cell_editor_mode(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    commands.init_resource::<CellEditorState>();

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
    commands.spawn(Cell::new_bundle(
        100.,
        Vec2::ZERO,
        Vec2::ZERO,
        Color::linear_rgb(0.5, 1.0, 0.5),
        &mut meshes,
        &mut materials,
    ));
}

pub fn exit_cell_editor_mode(mut commands: Commands, dishes: Query<Entity, With<DishMarker>>, cells: Query<Entity, With<Cell>>) {
    commands.remove_resource::<CellEditorState>();

    for entity in dishes {
        commands.entity(entity).despawn();
    }

    for entity in cells {
        commands.entity(entity).despawn();
    }
}

// ---------------------------------------------------------------------
