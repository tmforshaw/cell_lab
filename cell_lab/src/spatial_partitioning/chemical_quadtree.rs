use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

use crate::{
    game::{game_mode::GameMode, game_parameters::GameParameters},
    spatial_partitioning::quadtree::QuadTree,
};

use super::quadtree::QuadTreeTrait;

#[derive(Resource)]
pub struct ChemicalQuadTree(pub QuadTree<Entity>);

impl ChemicalQuadTree {}

impl Deref for ChemicalQuadTree {
    type Target = QuadTree<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ChemicalQuadTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl QuadTreeTrait<Entity> for ChemicalQuadTree {
    fn get_colour(&self, param: &GameParameters, _game_mode: &GameMode) -> Color {
        param.simulation_mode.chemical_quadtree.draw_colour
    }

    fn new_from_parameters(param: &GameParameters, _game_mode: &GameMode) -> Self {
        Self(QuadTree::new(
            Vec2::ZERO,
            param.simulation_mode.dish_parameters.size,
            param.simulation_mode.chemical_quadtree.max_depth,
            param.simulation_mode.chemical_quadtree.max_capacity_per_node,
        ))
    }
}

// Marker for whether to show quadtree
#[derive(Resource, Default)]
pub struct ShowChemicalQuadTree(pub bool);

impl Deref for ShowChemicalQuadTree {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Marker for the quadtree visualisation debug sprites
#[derive(Component, Default)]
pub struct ChemicalQuadTreeDebug;
