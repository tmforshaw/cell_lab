use bevy::prelude::*;

use crate::{
    game::game_mode::GameMode,
    simulation::dish::{Dish, DishBundle},
};

#[derive(Resource)]
pub struct GameParameters {
    pub collision_impulse_scale: f32,
    pub simulation_mode: SimulationModeParameters,
    pub cell_editor_mode: CellEditorModeParameters,
    pub cell_parameters: CellParameters,
    pub selection_parameters: SelectionParameters,
    pub ui_parameters: UiParameters,
    pub genome_mode_colour_offset: f32,
    pub cell_energy_decay_rate: f32,
}

pub struct SimulationModeParameters {
    pub dish_parameters: DishParameters,
    pub cell_quadtree: QuadtreeParameters,
    pub chemical_quadtree: QuadtreeParameters,
    pub chemical_parameters: ChemicalParameters,
    pub starting_cell_num: usize,
    pub cell_size_scale_factor: f32,
}

pub struct CellEditorModeParameters {
    pub dish_parameters: DishParameters,
    pub cell_quadtree: QuadtreeParameters,
    pub simulation_parameters: CellEditorSimulationParameters,
    pub cell_size_scale_factor: f32,
    pub max_editor_age: f32,
    pub editor_age_epsilon: f32,
    pub cell_energy_gain_rate: f32,
}

pub struct QuadtreeParameters {
    pub max_depth: usize,
    pub max_capacity_per_node: usize,
    pub draw_colour: Color,
}

pub struct CellParameters {
    pub starting_energy: f32,
    pub max_velocity: f32,
    pub max_split_age: f32,
    pub max_energy: f32,
    pub min_energy: f32,
    pub mass_energy_scale_power: f32,
    pub split_padding: f32,
}

pub struct ChemicalParameters {
    pub size: f32,
    pub energy: f32,
    pub spawn_rate: f32,
    pub max_instances: usize,
    pub colour: Color,
}

pub struct DishParameters {
    pub size: Vec2,
    pub colour: Color,
}

pub struct UiParameters {
    pub separator_spacing: f32,
    pub subsection_spacing: f32,
    pub slider_percent: f32,
    pub cell_editor_panel_width: f32,
}

pub struct CellEditorSimulationParameters {
    pub delta_time: f32,
    pub snapshot_frame_count_interval: usize,
    pub max_snapshot_num: usize,
}

pub struct SelectionParameters {
    pub colour: Color,
    pub scale: f32,
}

impl Default for GameParameters {
    fn default() -> Self {
        Self {
            collision_impulse_scale: 10.,
            simulation_mode: SimulationModeParameters::default(),
            cell_editor_mode: CellEditorModeParameters::default(),
            cell_parameters: CellParameters::default(),
            selection_parameters: SelectionParameters::default(),
            ui_parameters: UiParameters::default(),
            genome_mode_colour_offset: 120.,
            cell_energy_decay_rate: 1.0,
        }
    }
}

impl Default for SimulationModeParameters {
    fn default() -> Self {
        Self {
            dish_parameters: DishParameters {
                size: Vec2::splat(1200.),
                colour: Color::linear_rgb(0.2, 0.2, 0.2),
            },
            cell_quadtree: QuadtreeParameters {
                max_depth: 6,
                max_capacity_per_node: 8,
                draw_colour: Color::linear_rgba(0., 0., 1., 0.5),
            },
            chemical_quadtree: QuadtreeParameters {
                max_depth: 6,
                max_capacity_per_node: 8,
                draw_colour: Color::linear_rgba(1., 0., 1., 0.5),
            },
            chemical_parameters: ChemicalParameters::default(),
            starting_cell_num: 1,
            cell_size_scale_factor: 10.,
        }
    }
}

impl Default for CellEditorModeParameters {
    fn default() -> Self {
        Self {
            dish_parameters: DishParameters {
                size: Vec2::splat(1200.),
                colour: Color::linear_rgb(0.2, 0.2, 0.2),
            },
            cell_quadtree: QuadtreeParameters {
                max_depth: 6,
                max_capacity_per_node: 8,
                draw_colour: Color::linear_rgba(0., 0., 1., 0.5),
            },
            simulation_parameters: CellEditorSimulationParameters::default(),
            cell_size_scale_factor: 50.,
            max_editor_age: 30.,
            editor_age_epsilon: 0.02,
            cell_energy_gain_rate: 2.0,
        }
    }
}

impl Default for CellParameters {
    fn default() -> Self {
        Self {
            starting_energy: 10.,
            max_velocity: 100.,
            max_split_age: 25.,
            max_energy: 50.,
            min_energy: 2.,
            mass_energy_scale_power: 0.5,
            split_padding: 1.1, // Multipllier for offset of daughters from each other (Multiplies radius)
        }
    }
}

impl Default for ChemicalParameters {
    fn default() -> Self {
        Self {
            size: 10.,
            energy: 10.,
            spawn_rate: 50.,
            max_instances: 400,
            colour: Color::linear_rgba(0.5, 0.1, 0.1, 0.75),
        }
    }
}

impl Default for DishParameters {
    fn default() -> Self {
        Self {
            size: Vec2::splat(1200.),
            colour: Color::linear_rgb(0.2, 0.2, 0.2),
        }
    }
}

impl Default for UiParameters {
    fn default() -> Self {
        Self {
            separator_spacing: 8.,
            subsection_spacing: 4.,
            slider_percent: 0.45,
            cell_editor_panel_width: 600.,
        }
    }
}

impl Default for CellEditorSimulationParameters {
    fn default() -> Self {
        Self {
            delta_time: 1. / 60.,
            snapshot_frame_count_interval: 10,
            max_snapshot_num: 64,
        }
    }
}

impl Default for SelectionParameters {
    fn default() -> Self {
        Self {
            colour: Color::linear_rgba(1.0, 1.0, 0.0, 1.0),
            scale: 1.05,
        }
    }
}

impl GameParameters {
    #[must_use]
    pub const fn get_cell_size_scale(&self, game_mode: &GameMode) -> f32 {
        match game_mode {
            GameMode::Simulation => self.simulation_mode.cell_size_scale_factor,
            GameMode::CellEditor => self.cell_editor_mode.cell_size_scale_factor,
        }
    }
}

impl DishParameters {
    #[must_use]
    pub fn get_dish_bundle(&self) -> DishBundle {
        Dish::new_bundle(self.size, self.colour)
    }
}

impl UiParameters {
    #[must_use]
    pub fn get_cell_editor_slider_width(&self) -> f32 {
        self.slider_percent * self.cell_editor_panel_width
    }
}

impl CellEditorSimulationParameters {
    #[must_use]
    pub fn get_snapshot_interval(&self) -> f32 {
        self.delta_time * (self.snapshot_frame_count_interval as f32)
    }
}
