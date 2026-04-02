use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

use crate::{WINDOW_SIZE, spatial_partitioning::quadtree::QuadTree};

use super::quadtree::QuadTreeTrait;

const CHEMICAL_QUADTREE_SIZE: Vec2 = WINDOW_SIZE;
const CHEMICAL_QUADTREE_MAX_DEPTH: usize = 6;
const CHEMICAL_QUADTREE_MAX_CAPACITY_PER_NODE: usize = 8;
const CHEMICAL_QUADTREE_COLOUR: Color = Color::linear_rgba(1., 0., 1., 0.5);

#[derive(Resource)]
pub struct ChemicalQuadTree(pub QuadTree<Entity>);

impl Default for ChemicalQuadTree {
    fn default() -> Self {
        Self(QuadTree::new(
            Vec2::ZERO,
            CHEMICAL_QUADTREE_SIZE,
            CHEMICAL_QUADTREE_MAX_DEPTH,
            CHEMICAL_QUADTREE_MAX_CAPACITY_PER_NODE,
        ))
    }
}

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
    fn get_colour(&self) -> Color {
        CHEMICAL_QUADTREE_COLOUR
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
