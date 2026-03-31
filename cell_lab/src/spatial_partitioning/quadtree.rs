use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::collision::collider::aabb_contains_point;

const QUAD_TREE_SIZE: f32 = 1500.;
const QUAD_TREE_MAX_DEPTH: usize = 16;
const QUAD_TREE_CAPACITY: usize = 16;

pub struct QuadTreeNode {
    bounds: Aabb2d,
    points: Vec<(Entity, Vec2)>,
    children: Option<Box<[Self; 4]>>,
    depth: usize,
}

impl QuadTreeNode {
    #[must_use]
    pub const fn new(bounds: Aabb2d, depth: usize) -> Self {
        Self {
            bounds,
            points: Vec::new(),
            children: None,
            depth,
        }
    }

    pub fn subdivide(&mut self) {
        let centre = self.bounds.center();
        let half = self.bounds.half_size() * 0.5;

        self.children = Some(Box::new([
            Self::new(Aabb2d::new(centre + half * Vec2::new(-1., 1.), half), self.depth + 1), // Top-Left
            Self::new(Aabb2d::new(centre + half * Vec2::new(1., 1.), half), self.depth + 1),  // Top-Right
            Self::new(Aabb2d::new(centre + half * Vec2::new(-1., -1.), half), self.depth + 1), // Bottom-Left
            Self::new(Aabb2d::new(centre + half * Vec2::new(1., -1.), half), self.depth + 1), // Bottom-Right
        ]));
    }

    pub fn insert(&mut self, entity: Entity, position: Vec2) -> bool {
        if !aabb_contains_point(&self.bounds, position) {
            return false;
        }

        if self.points.len() < QUAD_TREE_CAPACITY || self.depth >= QUAD_TREE_MAX_DEPTH {
            self.points.push((entity, position));

            return true;
        }

        if self.children.is_none() {
            self.subdivide();
        }

        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                if child.insert(entity, position) {
                    return true;
                }
            }
        }

        false
    }

    pub fn query(&self, bounds: &Aabb2d, out: &mut Vec<Entity>) {
        if !self.bounds.intersects(bounds) {
            return;
        }

        for (entity, position) in &self.points {
            if aabb_contains_point(bounds, *position) {
                out.push(*entity);
            }
        }

        if let Some(children) = &self.children {
            for child in children.iter() {
                child.query(bounds, out);
            }
        }
    }

    #[must_use]
    pub fn build_tree(entities_and_transforms: Vec<(Entity, Transform)>) -> Self {
        let bounds = Aabb2d::new(Vec2::ZERO, Vec2::splat(QUAD_TREE_SIZE));

        let mut root = Self::new(bounds, 0);

        for (entity, transform) in entities_and_transforms {
            root.insert(entity, transform.translation.xy());
        }

        root
    }
}
