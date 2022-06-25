use std::borrow::Cow;

use bevy::prelude::*;

use crate::components::look_description::LookDescription;

/// Mark the entity as item
#[derive(Component)]
pub struct Item;

#[derive(Bundle)]
pub struct ItemBundle {
    item: Item,
    name: Name,
    look_description: LookDescription,
}

impl ItemBundle {
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        look_description: impl Into<Cow<'static, str>>,
    ) -> Self {
        ItemBundle {
            item: Item,
            name: Name::new(name),
            look_description: LookDescription::new(look_description),
        }
    }
}
