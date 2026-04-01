use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CellEditorDialogState {
    pub save_dialog_open: bool,
    pub save_text: String,
    pub load_dialog_open: bool,
}
