use bevy::prelude::*;

#[derive(Component)]
pub struct PendingDespawn;

pub fn apply_pending_despawns(mut commands: Commands, pending: Query<Entity, With<PendingDespawn>>) {
    for entity in pending {
        commands.entity(entity).despawn();
    }
}
