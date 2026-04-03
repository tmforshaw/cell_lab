use bevy::prelude::*;

use crate::{
    cell_editor::{
        logical_cell::{LogicalCell, create_root_logical_cell, resolve_logical_cell_collision},
        snapshot::{CellEditorSimulationState, CellHistoryCache},
        state::CellEditorState,
    },
    cells::{CellMaterial, Velocity, cell::CellBundle},
    game::{game_mode::GameMode, game_parameters::GameParameters},
    genomes::{CellSplitType, GenomeBank, daughters::DaughterData},
};

#[derive(States, Debug, Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum CellEditorSimulationStatus {
    #[default]
    NeedsRecompute,
    Clean,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct CellEditorSimulationClearMessage;

#[allow(clippy::needless_pass_by_value)]
pub fn clear_simulation_cache_message_reader(
    events: MessageReader<CellEditorSimulationClearMessage>,
    mut sim_status: ResMut<NextState<CellEditorSimulationStatus>>,
    mut sim: ResMut<CellEditorSimulationState>,
    mut cache: ResMut<CellHistoryCache>,
) {
    // Clear the simulation cache if this event is found
    if !events.is_empty() {
        *sim = CellEditorSimulationState::default();
        cache.clear();

        // Mark the simulation as needing recomputing
        sim_status.set(CellEditorSimulationStatus::NeedsRecompute);
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn simulate_to_editor_age(
    mut sim: ResMut<CellEditorSimulationState>,
    state: Res<CellEditorState>,
    param: Res<GameParameters>,
    game_mode: Res<State<GameMode>>,
    mut cache: ResMut<CellHistoryCache>,
    genome_bank: Res<GenomeBank>,
) {
    let target_time = state.editor_age.get_age();
    let dt = param.cell_editor_mode.simulation_parameters.delta_time;

    // Load best snapshot, or reset
    if let Some(snapshot) = cache.get_closest_past_snapshot(target_time) {
        sim.cells = snapshot.cells.clone();
        sim.current_time = snapshot.time;
    } else {
        sim.cells = vec![create_root_logical_cell(&state, &param, &game_mode, &genome_bank)];
        sim.current_time = 0.0;
    }

    // Simulate forward from the selected time
    while sim.current_time < target_time {
        let current_time = sim.current_time;
        step_simulation(&mut sim.cells, dt, current_time, &param, &game_mode, &genome_bank);

        sim.current_time += dt;

        // Cache snapshot if needed
        if cache.should_store_snapshot(sim.current_time, &param) {
            cache.insert(&sim.cells, sim.current_time);

            cache.trim(&param);
        }
    }
}

pub fn step_simulation(
    cells: &mut Vec<LogicalCell>,
    dt: f32,
    current_time: f32,
    param: &GameParameters,
    game_mode: &GameMode,
    genome_bank: &GenomeBank,
) {
    // Update age, energy, and size
    let mut i = 0;
    while i < cells.len() {
        let lc = &mut cells[i];
        lc.cell.age = current_time - lc.time_of_birth;

        // Cell will die from lack of energy
        if lc.cell.energy <= param.cell_parameters.min_energy {
            cells.swap_remove(i);
        } else {
            lc.cell.energy += ((param.cell_editor_mode.cell_energy_gain_rate - param.cell_energy_decay_rate) * dt)
                .min(param.cell_parameters.max_energy);
            lc.transform.scale = lc.cell.get_size(param, game_mode).extend(1.);

            i += 1; // Only advance if nothing was removed
        }
    }

    // Movement
    for lc in cells.iter_mut() {
        // Velocity integration
        lc.transform.translation += (lc.velocity.0 * dt).extend(0.);

        let pos = &mut lc.transform.translation;
        let vel = &mut lc.velocity.0;
        let cell_size = lc.cell.get_size(param, game_mode) * 0.5;

        let half_bounds_size = param.cell_editor_mode.dish_parameters.size * 0.5;

        // Editor bounds reflect velocity

        // Horizontal Bounds
        if pos.x.abs() > half_bounds_size.x - cell_size.x {
            pos.x = pos.x.signum() * (half_bounds_size.x - cell_size.x);
            vel.x = -vel.x;
        }

        // Vertical Bounds
        if pos.y.abs() > half_bounds_size.y - cell_size.y {
            pos.y = pos.y.signum() * (half_bounds_size.y - cell_size.y);
            vel.y = -vel.y;
        }
    }

    // Resolve Collisions
    resolve_logical_cell_collision(cells, param, game_mode);

    // Splitting
    let mut new_cells = Vec::new();
    let mut i = 0;
    while i < cells.len() {
        let genome_mode = cells[i].cell.get_genome_mode(genome_bank);

        if match genome_mode.split_type {
            CellSplitType::Energy => cells[i].cell.energy >= genome_mode.split_energy,
            CellSplitType::Age => cells[i].cell.age >= genome_mode.split_age,
            CellSplitType::Never => false,
        } {
            // Remove parent from cells Vec
            let parent = cells.swap_remove(i);

            // Get data for daughters
            let (d1, d2) = DaughterData::get_from_parent(
                &parent.cell,
                &parent.velocity,
                &parent.transform,
                param,
                game_mode,
                genome_bank,
            );

            let time_of_birth = current_time;

            // Add daughter 1 to cell Vec
            new_cells.push(LogicalCell {
                cell: d1.cell,
                transform: d1.transform,
                velocity: Velocity(d1.velocity),
                time_of_birth,
            });

            // Add daughter 2 to cell Vec
            new_cells.push(LogicalCell {
                cell: d2.cell,
                transform: d2.transform,
                velocity: Velocity(d2.velocity),
                time_of_birth,
            });
        } else {
            i += 1; // Only advance if nothing was removed
        }
    }

    // Add the new cells to the cells Vec
    cells.extend(new_cells);
}

#[allow(clippy::needless_pass_by_value)]
pub fn spawn_cells_from_simulation(
    mut commands: Commands,
    sim: Res<CellEditorSimulationState>,
    mut sim_status: ResMut<NextState<CellEditorSimulationStatus>>,
    genome_bank: Res<GenomeBank>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    // Iterate through logical cells and spawn them in
    for lc in &sim.cells {
        let genome_mode = lc.cell.get_genome_mode(&genome_bank);

        commands.spawn(CellBundle::new(
            lc.cell.clone(),
            lc.velocity.clone(),
            lc.transform,
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(CellMaterial::new(
                genome_mode.colour,
                true,
                genome_mode.split_angle,
                genome_mode.split_fraction,
            ))),
        ));
    }

    // Mark the simulation as clean
    sim_status.set(CellEditorSimulationStatus::Clean);
}
