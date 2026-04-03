use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    cell_editor::{events::SelectedCell, state::CellEditorState},
    cells::Cell,
    despawning::PendingDespawn,
    genomes::GenomeCollection,
};

const SPLIT_ARROW_WIDTH: f32 = 0.05;
const SPLIT_ARROW_SEGMENTS: usize = 6;
const SPLIT_ARROW_GAP_PERCENT: f32 = 0.33;
const SPLIT_ARROW_COLOUR: Color = Color::linear_rgb(1.0, 1.0, 1.0);

#[derive(Component)]
pub struct SplitAngleArrow;

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub fn draw_cell_info(
    mut commands: Commands,
    genome_collection: Res<GenomeCollection>,
    state: Res<CellEditorState>,
    cells: Query<(Entity, &Cell), (Added<SelectedCell>, Without<PendingDespawn>)>,
) {
    for (entity, cell) in cells {
        // TODO Move split angle arrow to show split fraction
        // Draw the split angle
        draw_split_angle_arrow_as_child(&mut commands, &genome_collection, &state, entity, cell);

        // TODO Draw split direction
    }
}

pub fn draw_split_angle_arrow_as_child(
    commands: &mut Commands,
    genome_collection: &GenomeCollection,
    state: &Res<CellEditorState>,
    entity: Entity,
    cell: &Cell,
) {
    // Calculate the length and direction for the split angle
    let dir = Vec2::Y.rotate(Vec2::from_angle(
        state.get_selected_genome(genome_collection)[cell.genome_mode_id].split_angle,
    ));

    let start = -dir / 2.;

    // Compute dash length that evenly divides the line
    let dash_length = (1.0 - SPLIT_ARROW_GAP_PERCENT) / (SPLIT_ARROW_SEGMENTS as f32 - SPLIT_ARROW_GAP_PERCENT);
    let gap_length = dash_length * SPLIT_ARROW_GAP_PERCENT / (1.0 - SPLIT_ARROW_GAP_PERCENT);

    // Draw each dash
    for i in 0..SPLIT_ARROW_SEGMENTS {
        let seg_start = start + dir * (i as f32 * (dash_length + gap_length));
        let seg_end = seg_start + dir * dash_length;

        let line = shapes::Line(seg_start, seg_end);

        // Spawn as a child of this cell
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                ShapeBuilder::with(&line)
                    .stroke((SPLIT_ARROW_COLOUR, SPLIT_ARROW_WIDTH))
                    .build(),
                Transform::from_xyz(0., 0., 2.),
                SplitAngleArrow,
            ));
        });
    }
}
