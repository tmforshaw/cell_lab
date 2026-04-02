use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CellEditorDialogState {
    pub save_dialog_open: bool,
    pub save_text: String,
    pub load_dialog_open: bool,
    pub load_selected_file: Option<usize>,
    pub delete_dialog_open: bool,
    pub delete_file: Option<String>,
}
