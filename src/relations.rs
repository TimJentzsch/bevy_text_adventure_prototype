use bevy::prelude::*;

/// Establishes a connection between the two location entities.
///
/// The connection is symmetric, i.e. it does not matter which location is the first or second element.
#[derive(Component)]
pub struct LocationConnection(pub Entity, pub Entity);

impl LocationConnection {
    /// Return the location that is connected to the given location.
    ///
    /// Returns [`None`] if the location does not appear in this connection.
    pub fn connected_location(&self, location: Entity) -> Option<Entity> {
        if location == self.0 {
            Some(self.1)
        } else if location == self.1 {
            Some(self.0)
        } else {
            None
        }
    }
}
