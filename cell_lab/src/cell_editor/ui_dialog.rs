use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CellEditorUiDialogState {
    save_dialog_open: bool,
    pub save_filename: String,
    overwrite_dialog_open: bool,
    load_dialog_open: bool,
    pub load_selected_file: Option<usize>,
    delete_dialog_open: bool,
    pub delete_file: Option<String>,
}

impl CellEditorUiDialogState {
    #[must_use]
    pub const fn save_dialog_is_open(&self) -> bool {
        self.save_dialog_open
    }

    #[must_use]
    pub const fn overwrite_dialog_is_open(&self) -> bool {
        self.overwrite_dialog_open
    }

    #[must_use]
    pub const fn load_dialog_is_open(&self) -> bool {
        self.load_dialog_open
    }

    #[must_use]
    pub const fn delete_dialog_is_open(&self) -> bool {
        self.delete_dialog_open
    }

    pub fn open_save_dialog(&mut self) {
        // Open save dialog, everything else gets cleared
        *self = Self {
            save_dialog_open: true,
            ..default()
        };
    }

    pub fn open_overwrite_dialog(&mut self) {
        // Open overwrite dialog, everything except save_filename gets cleared
        *self = Self {
            overwrite_dialog_open: true,
            save_filename: self.save_filename.clone(),
            ..default()
        };
    }

    pub fn open_load_dialog(&mut self) {
        // Open load dialog, everything else gets cleared
        *self = Self {
            load_dialog_open: true,
            ..default()
        };
    }

    pub fn open_delete_dialog<S: AsRef<str>>(&mut self, delete_file: S) {
        // Open delete dialog, set delete_file, close the load dialog, keeping the selected file the same
        *self = Self {
            delete_dialog_open: true,
            delete_file: Some(delete_file.as_ref().to_string()),
            load_dialog_open: false,
            load_selected_file: self.load_selected_file,
            ..default()
        };
    }

    pub fn close_all_dialogs(&mut self) {
        *self = Self::default();
    }

    pub fn close_delete_dialog(&mut self) {
        // Close delete dialog, open the load dialog, keeping the selected file the same
        *self = Self {
            delete_dialog_open: false,
            load_dialog_open: true,
            load_selected_file: self.load_selected_file,
            ..default()
        };
    }
}
