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

/// Indicates that the item is available at the given location.
///
/// The first entity should have an [`Item`] component and the second entity should have a [`Location`] component.
#[derive(Component)]
pub struct ItemAtLocation(pub Entity, pub Entity);

impl ItemAtLocation {
    /// Gives the item entity if it is at the given location or [`None`] otherwise.
    pub fn available_item(&self, location: Entity) -> Option<Entity> {
        if location == self.1 {
            Some(self.0)
        } else {
            None
        }
    }
}
