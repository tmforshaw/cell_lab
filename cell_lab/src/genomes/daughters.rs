use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    cells::{CELL_SPLIT_PADDING, Cell, CellMaterial, Velocity, cell::CellBundle},
    genomes::GenomeCollection,
};

pub struct DaughterData {
    pub cell: Cell,
    pub velocity: Vec2,
    pub transform: Transform,
}

impl DaughterData {
    #[must_use]
    pub fn get_from_parent(
        parent: &Cell,
        velocity: &Velocity,
        transform: &Transform,
        genome_collection: &GenomeCollection,
    ) -> (Self, Self) {
        let parent_genome = &genome_collection[parent.genome_bank_id][parent.genome_id];

        // Split energy depending on split fraction
        let d1_energy = parent.energy * parent_genome.split_fraction;
        let d2_energy = parent.energy - d1_energy;

        // Set genome_id according to genome bank
        let d1_genome_id = parent_genome.daughter_genomes.0;
        let d2_genome_id = parent_genome.daughter_genomes.1;

        // Also ensure to rotate the split direction based on the parent's rotation
        let parent_angle = transform.rotation.to_euler(EulerRot::XYZ).2;

        // Give new velocity depending on split angle
        let velocity_mag = parent_genome.split_force;
        let d1_new_velocity = velocity_mag * Vec2::Y.rotate(Vec2::from_angle(parent_genome.split_angle - PI / 2. + parent_angle));
        let d2_new_velocity = velocity_mag * Vec2::Y.rotate(Vec2::from_angle(parent_genome.split_angle + PI / 2. + parent_angle));

        // Calculate the amount of velocity to give to each daughter based on split fraction
        let d1_velocity_from_parent = velocity.0 * parent_genome.split_fraction.sqrt();
        let d2_velocity_from_parent = velocity.0 * (1. - parent_genome.split_fraction).sqrt();

        // Add up the daughter's new velocity and the velocity from the parent
        let d1_velocity = d1_new_velocity + d1_velocity_from_parent;
        let d2_velocity = d2_new_velocity + d2_velocity_from_parent;

        // Offset the daughters by their width (plus a little bit of padding)
        let d1_position = transform.translation.xy()
            + (transform.scale.xy() * parent_genome.split_fraction) / 2. * d1_new_velocity.normalize() * CELL_SPLIT_PADDING;
        let d2_position = transform.translation.xy()
            + (transform.scale.xy() * (1. - parent_genome.split_fraction)) / 2.
                * d2_new_velocity.normalize()
                * CELL_SPLIT_PADDING;

        // Convert this information into Cells for the daughters
        let (d1_cell, d2_cell) = (
            Cell {
                energy: d1_energy,
                age: 0.0,
                genome_id: d1_genome_id,
                genome_bank_id: parent.genome_bank_id,
                size_per_mass: parent.size_per_mass,
            },
            Cell {
                energy: d2_energy,
                age: 0.0,
                genome_id: d2_genome_id,
                genome_bank_id: parent.genome_bank_id,
                size_per_mass: parent.size_per_mass,
            },
        );

        // Get the scale of the cells
        let d1_scale = d1_cell.get_size();
        let d2_scale = d2_cell.get_size();

        let parent_rotation_z = transform.rotation.to_euler(EulerRot::XYZ).2;

        // Compute the daughter's transforms based on the calculated information
        let (d1_transform, d2_transform) = (
            Transform::from_translation(d1_position.extend(transform.translation.z))
                .with_rotation(Quat::from_rotation_z(parent_genome.daughter_angles.0 + parent_rotation_z))
                .with_scale(d1_scale.extend(1.0)),
            Transform::from_translation(d2_position.extend(transform.translation.z))
                .with_rotation(Quat::from_rotation_z(parent_genome.daughter_angles.1 + parent_rotation_z))
                .with_scale(d2_scale.extend(1.0)),
        );

        (
            // Set the first daughter's parameters
            Self {
                cell: d1_cell,
                velocity: d1_velocity,
                transform: d1_transform,
            },
            // Set the second daughter's parameters
            Self {
                cell: d2_cell,
                velocity: d2_velocity,
                transform: d2_transform,
            },
        )
    }

    pub fn into_cell_bundle(
        &self,
        genome_collection: &GenomeCollection,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<CellMaterial>,
    ) -> CellBundle {
        CellBundle::new(
            self.cell.clone(),
            Velocity(self.velocity),
            self.transform,
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(CellMaterial::new(self.cell.get_genome(genome_collection).colour))),
        )
    }
}
