use bevy::{math::bounding::BoundingVolume, prelude::*};

use crate::{despawning::PendingDespawn, spatial_partitioning::quadtree::QuadtreeDebug};

use super::quadtree::QuadTree;

const CELL_QUADTREE_SIZE: Vec2 = Vec2::splat(1200.);
const CELL_QUADTREE_MAX_DEPTH: usize = 4;
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

fn spawn_cell_quadtree_line(commands: &mut Commands, pos: Vec2, size: Vec2) {
    commands.spawn((
        (
            Sprite {
                color: CELL_QUADTREE_COLOUR,
                custom_size: Some(size),
                ..default()
            },
            Transform::from_translation(pos.extend(0.0)),
        ),
        QuadtreeDebug,
    ));
}

// Whether to show quadtree or not
#[derive(Resource, Default)]
pub struct ShowCellQuadTree(pub bool);

#[allow(clippy::needless_pass_by_value)]
pub fn visualize_cell_quadtree(
    mut commands: Commands,
    cell_quadtree: ResMut<CellQuadTree>,
    show_cell_quadtree: Res<ShowCellQuadTree>,
    query_existing: Query<Entity, With<QuadtreeDebug>>,
) {
    // remove old debug visuals
    for e in &query_existing {
        commands.entity(e).insert(PendingDespawn);
    }

    // Only show if the condition is set
    if show_cell_quadtree.0 {
        // Collect all node bounds
        let mut rects = cell_quadtree.0.collect_bounds();

        rects.reverse();
        rects.pop();

        // Spawn sprites for each
        for aabb in rects {
            let centre = aabb.center();
            let size = aabb.half_size() * 2.;

            let thickness = 8.0;

            let hw = size.x * 0.5;
            let hh = size.y * 0.5;

            // Top
            spawn_cell_quadtree_line(&mut commands, centre + Vec2::new(0.0, hh), Vec2::new(size.x, thickness));

            // Bottom
            spawn_cell_quadtree_line(&mut commands, centre + Vec2::new(0.0, -hh), Vec2::new(size.x, thickness));

            // Left
            spawn_cell_quadtree_line(&mut commands, centre + Vec2::new(-hw, 0.0), Vec2::new(thickness, size.y));

            // Right
            spawn_cell_quadtree_line(&mut commands, centre + Vec2::new(hw, 0.0), Vec2::new(thickness, size.y));
        }
    }
}
