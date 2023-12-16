use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    UserInput,
    EntityUpdates,
    CollisionDetection,
    DespawnEntities,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
       app.configure_sets(
            Update, 
            (
                InGameSet::DespawnEntities,
                // Flush Commands
                InGameSet::UserInput,
                InGameSet::EntityUpdates,
                InGameSet::CollisionDetection,
            ).chain()
        )
        // Manually adding a flush point
        .add_systems(
            Update,
            apply_deferred
            .after(InGameSet::DespawnEntities)
            .before(InGameSet::UserInput),
        );
    }
}
