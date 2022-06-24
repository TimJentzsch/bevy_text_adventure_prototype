//! Derived from [`Name`](bevy::prelude::Name).
use std::{borrow::Cow, ops::Deref};

use bevy::prelude::*;

/// A description to show for the entity when using the LOOK command.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct LookDescription {
    description: Cow<'static, str>,
}

impl Default for LookDescription {
    fn default() -> Self {
        LookDescription::new("")
    }
}

impl LookDescription {
    /// Creates a new [`LookDescription`] from any string-like type.
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        let name = name.into();
        LookDescription { description: name }
    }

    /// Sets the entity's LOOK description.
    #[inline(always)]
    pub fn set(&mut self, name: impl Into<Cow<'static, str>>) {
        *self = LookDescription::new(name);
    }

    /// Updates the LOOK description of the entity in place.
    ///
    /// This will allocate a new string if the description was previously
    /// created from a borrow.
    #[inline(always)]
    pub fn mutate<F: FnOnce(&mut String)>(&mut self, f: F) {
        f(self.description.to_mut());
    }

    /// Gets the LOOK description of the entity as a `&str`.
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        &self.description
    }
}

impl std::fmt::Display for LookDescription {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.description, f)
    }
}

/* Conversions from strings */

impl From<&str> for LookDescription {
    #[inline(always)]
    fn from(name: &str) -> Self {
        LookDescription::new(name.to_owned())
    }
}
impl From<String> for LookDescription {
    #[inline(always)]
    fn from(name: String) -> Self {
        LookDescription::new(name)
    }
}

/* Conversions to strings */

impl AsRef<str> for LookDescription {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        &self.description
    }
}
impl From<&LookDescription> for String {
    #[inline(always)]
    fn from(val: &LookDescription) -> String {
        val.as_str().to_owned()
    }
}
impl From<LookDescription> for String {
    #[inline(always)]
    fn from(val: LookDescription) -> String {
        val.description.into_owned()
    }
}

impl Deref for LookDescription {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.description.as_ref()
    }
}
