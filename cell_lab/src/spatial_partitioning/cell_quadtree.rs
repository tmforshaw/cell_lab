use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

use crate::spatial_partitioning::quadtree::QuadTree;

use super::quadtree::QuadTreeTrait;

const CELL_QUADTREE_SIZE: Vec2 = Vec2::splat(1600.);
const CELL_QUADTREE_MAX_DEPTH: usize = 6;
const CELL_QUADTREE_MAX_CAPACITY_PER_NODE: usize = 8;
const CELL_QUADTREE_COLOUR: Color = Color::linear_rgba(0., 0., 1., 0.5);

#[derive(Resource)]
pub struct CellQuadTree(pub QuadTree);

impl Default for CellQuadTree {
    fn default() -> Self {
        Self(QuadTree::new(
            Vec2::ZERO,
            CELL_QUADTREE_SIZE,
            CELL_QUADTREE_MAX_DEPTH,
            CELL_QUADTREE_MAX_CAPACITY_PER_NODE,
        ))
    }
}

impl Deref for CellQuadTree {
    type Target = QuadTree;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CellQuadTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl QuadTreeTrait for CellQuadTree {
    fn get_colour(&self) -> Color {
        CELL_QUADTREE_COLOUR
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
