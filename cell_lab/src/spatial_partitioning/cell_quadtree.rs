use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

use crate::{
    game::{game_mode::GameMode, game_parameters::GameParameters},
    spatial_partitioning::quadtree::QuadTree,
};

use super::quadtree::QuadTreeTrait;

#[derive(Resource)]
pub struct CellQuadTree(pub QuadTree<Entity>);

impl CellQuadTree {}

impl Deref for CellQuadTree {
    type Target = QuadTree<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CellQuadTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl QuadTreeTrait<Entity> for CellQuadTree {
    fn get_colour(&self, param: &GameParameters, game_mode: &GameMode) -> Color {
        match game_mode {
            GameMode::Simulation => param.simulation_mode.cell_quadtree.draw_colour,
            GameMode::CellEditor => param.cell_editor_mode.cell_quadtree.draw_colour,
        }
    }

    fn new_from_parameters(param: &GameParameters, game_mode: &GameMode) -> Self {
        Self(match game_mode {
            GameMode::Simulation => QuadTree::new(
                Vec2::ZERO,
                param.simulation_mode.dish_parameters.size,
                param.simulation_mode.cell_quadtree.max_depth,
                param.simulation_mode.cell_quadtree.max_capacity_per_node,
            ),
            GameMode::CellEditor => QuadTree::new(
                Vec2::ZERO,
                param.cell_editor_mode.dish_parameters.size,
                param.cell_editor_mode.cell_quadtree.max_depth,
                param.cell_editor_mode.cell_quadtree.max_capacity_per_node,
            ),
        })
    }
}

// Marker for whether to show quadtree
#[derive(Resource, Default)]
pub struct ShowCellQuadTree(pub bool);

impl Deref for ShowCellQuadTree {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Marker for the quadtree visualisation debug sprites
#[derive(Component, Default)]
pub struct CellQuadTreeDebug;
