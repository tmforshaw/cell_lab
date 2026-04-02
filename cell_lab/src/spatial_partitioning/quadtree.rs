use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::collision::systems::aabb_contains_point;

pub struct QuadTreeNode<Id: Copy> {
    bounds: Aabb2d,
    points: Vec<(Id, Vec2)>,
    children: Option<Box<[Self; 4]>>,
    depth: usize,
}

impl<Id: Copy> QuadTreeNode<Id> {
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

    pub fn insert(&mut self, identifier: Id, position: Vec2, node_capacity: usize, max_depth: usize) -> bool {
        if !aabb_contains_point(&self.bounds, position) {
            return false;
        }

        if self.points.len() < node_capacity || self.depth >= max_depth {
            self.points.push((identifier, position));

            return true;
        }

        if self.children.is_none() {
            self.subdivide();
        }

        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                if child.insert(identifier, position, node_capacity, max_depth) {
                    return true;
                }
            }
        }

        false
    }

    pub fn query(&self, bounds: &Aabb2d, out: &mut Vec<Id>) {
        if !self.bounds.intersects(bounds) {
            return;
        }

        for (identifier, position) in &self.points {
            if aabb_contains_point(bounds, *position) {
                out.push(*identifier);
            }
        }

        if let Some(children) = &self.children {
            for child in children.iter() {
                child.query(bounds, out);
            }
        }
    }
}

pub struct QuadTree<I: Copy> {
    pub root: QuadTreeNode<I>,
    pub max_depth: usize,
    pub node_capacity: usize,
}

impl<Id: Copy> QuadTree<Id> {
    #[must_use]
    pub fn new(centre: Vec2, size: Vec2, max_depth: usize, max_capacity_per_node: usize) -> Self {
        Self {
            root: QuadTreeNode::new(Aabb2d::new(centre, size * 0.5), 0),
            max_depth,
            node_capacity: max_capacity_per_node,
        }
    }

    pub fn build(&mut self, identifiers_and_transforms: &Vec<(Id, Transform)>) {
        for (identifier, transform) in identifiers_and_transforms {
            self.root
                .insert(*identifier, transform.translation.xy(), self.node_capacity, self.max_depth);
        }
    }

    #[must_use]
    pub fn build_new(
        centre: Vec2,
        size: Vec2,
        max_depth: usize,
        max_capacity_per_node: usize,
        identifiers_and_transforms: &Vec<(Id, Transform)>,
    ) -> Self {
        let mut new = Self::new(centre, size, max_depth, max_capacity_per_node);

        new.build(identifiers_and_transforms);

        new
    }

    #[must_use]
    pub const fn get_root(&self) -> &QuadTreeNode<Id> {
        &self.root
    }

    #[must_use]
    pub const fn get_root_mut(&mut self) -> &mut QuadTreeNode<Id> {
        &mut self.root
    }

    // Collect all the node bounds using Breadth-First Search
    #[must_use]
    pub fn collect_bounds(&self) -> Vec<Aabb2d> {
        let mut out = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_back(&self.root);

        while let Some(node) = queue.pop_front() {
            out.push(node.bounds);

            if let Some(children) = &node.children {
                for child in children.iter() {
                    queue.push_back(child);
                }
            }
        }

        out
    }
}

pub trait QuadTreeTrait<Id: Copy>: Deref<Target = QuadTree<Id>> + DerefMut<Target = QuadTree<Id>> {
    fn get_colour(&self) -> Color;
}

pub fn spawn_quadtree_line<D: Component + Default>(commands: &mut Commands, pos: Vec2, size: Vec2, colour: Color) {
    commands.spawn((
        (
            Sprite {
                color: colour,
                custom_size: Some(size),
                ..default()
            },
            Transform::from_translation(pos.extend(0.0)),
        ),
        D::default(),
    ));
}

pub trait QuadTreeData {}
