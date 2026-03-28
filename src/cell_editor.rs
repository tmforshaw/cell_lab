use bevy::prelude::*;

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
