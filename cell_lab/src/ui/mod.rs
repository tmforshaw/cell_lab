pub mod button;
pub mod checkbox;
pub mod colour_picker;
pub mod combobox;
pub mod dialog_events;
pub mod dialog_state;
pub mod directional_node;
pub mod label;
pub mod radio;
pub mod separator;
pub mod slider;
pub mod text_input;
pub mod ui_build;
pub mod ui_theme;
pub mod ui_theme_colour_palette;
pub mod window;

// UI Elements ------------------------------------------------------------------------------------------------------------------

pub use button::{
    button_events::{ButtonEvent, button_event_reader},
    button_systems::{ButtonId, button_interaction_system, spawn_button},
};

pub use checkbox::{
    checkbox_events::{CheckboxEvent, checkbox_event_reader},
    checkbox_systems::{CheckboxId, checkbox_interaction_system, spawn_checkbox},
};

pub use colour_picker::{
    colour_conversion::{rgb_to_hsv, rgba_to_hsva},
    colour_picker_events::{ColourPickerEvent, colour_picker_event_reader},
    colour_picker_material::ColourPickerMaterial,
    colour_picker_systems::{ColourPickerId, colour_picker_interaction_system, spawn_colour_picker},
};

pub use combobox::{
    combobox_events::{ComboboxEvent, combobox_event_reader},
    combobox_systems::{
        ComboboxId, combobox_option_select_system, combobox_text_update_system, combobox_toggle_system, spawn_combobox,
    },
};

pub use radio::{
    radio_events::{RadioEvent, radio_event_reader},
    radio_systems::{RadioId, radio_interaction_system, spawn_radio_buttonlike, spawn_radio_textlike},
};

pub use slider::{
    slider_events::{SliderEvent, slider_event_reader},
    slider_material::SliderHueMaterial,
    slider_systems::{
        SliderId, slider_begin_drag_system, slider_drag_system, slider_interaction_system, slider_release_system, spawn_slider,
        spawn_slider_with_material,
    },
};

pub use text_input::{
    text_input_events::{TextInputEvent, text_input_event_reader},
    text_input_systems::{
        TextInput, TextInputId, spawn_text_input, text_input_interaction_system, text_input_typing_system,
        text_input_update_display_system,
    },
};

// ------------------------------------------------------------------------------------------------------------------------------

pub use dialog_events::{SaveFilenameEvent, save_filename_event_reader};
pub use dialog_state::{UiDialogState, UiSaveDialogState, open_or_close_dialogs, spawn_save_dialog};
pub use directional_node::{spawn_horizontal, spawn_vertical};
pub use label::{spawn_heading, spawn_label, spawn_subheading};
pub use separator::{spawn_semi_separator, spawn_separator};
pub use ui_build::{UiRebuildState, build_ui};
pub use ui_theme::UiTheme;
pub use window::{UiPanelType, UiWindowId, spawn_dialog, spawn_panel};
