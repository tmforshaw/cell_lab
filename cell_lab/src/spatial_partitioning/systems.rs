use std::ops::Deref;

use bevy::{math::bounding::BoundingVolume, prelude::*};

use crate::{
    despawning::PendingDespawn,
    spatial_partitioning::quadtree::{QuadTreeData, QuadTreeTrait, spawn_quadtree_line},
};

#[allow(clippy::needless_pass_by_value)]
pub fn visualise_quadtree<T, S, D>(
    mut commands: Commands,
    quadtree: Res<T>,
    show_quadtree: Res<S>,
    query_existing: Query<Entity, (With<D>, Without<PendingDespawn>)>,
) where
    T: QuadTreeTrait + Resource,
    S: Resource + Deref<Target = bool>,
    D: Component + Default,
{
    // Remove previous lines
    for e in query_existing {
        commands.entity(e).insert(PendingDespawn);
    }

    // Only show if the condition is set
    if **show_quadtree {
        // Collect all node bounds
        let rects = (*quadtree).collect_bounds();

        let colour = quadtree.get_colour();
        let line_thickness = 8.0;

        // Spawn sprites for each line
        for aabb in rects {
            let centre = aabb.center();
            let size = aabb.half_size() * 2.;

            let hw = size.x * 0.5;
            let hh = size.y * 0.5;

            // Top
            spawn_quadtree_line::<D>(
                &mut commands,
                centre + Vec2::new(0.0, hh),
                Vec2::new(size.x, line_thickness),
                colour,
            );

            // Bottom
            spawn_quadtree_line::<D>(
                &mut commands,
                centre + Vec2::new(0.0, -hh),
                Vec2::new(size.x, line_thickness),
                colour,
            );

            // Left
            spawn_quadtree_line::<D>(
                &mut commands,
                centre + Vec2::new(-hw, 0.0),
                Vec2::new(line_thickness, size.y),
                colour,
            );

            // Right
            spawn_quadtree_line::<D>(
                &mut commands,
                centre + Vec2::new(hw, 0.0),
                Vec2::new(line_thickness, size.y),
                colour,
            );
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn build_quadtree<T: QuadTreeTrait + Resource + Default, I: Component + QuadTreeData>(
    mut quadtree: ResMut<T>,
    items: Query<(Entity, &Transform), (With<I>, Without<PendingDespawn>)>,
) {
    // Turn query into Vec of entities and transforms
    let mut entities_and_transforms = Vec::new();
    for (entity, &transform) in &items {
        entities_and_transforms.push((entity, transform));
    }

    // Build the quadtree
    *quadtree = T::default();
    (*quadtree).build(&entities_and_transforms);
}
