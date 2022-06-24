//! Derived from [`Name`](bevy::prelude::Name).
use std::{borrow::Cow, ops::Deref};

use bevy::prelude::*;

/// A description to show for the entity when using the GO command.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct GoDescription {
    description: Cow<'static, str>,
}

impl Default for GoDescription {
    fn default() -> Self {
        GoDescription::new("")
    }
}

impl GoDescription {
    /// Creates a new [`GoDescription`] from any string-like type.
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        let name = name.into();
        GoDescription { description: name }
    }

    /// Sets the entity's GO description.
    #[inline(always)]
    pub fn set(&mut self, name: impl Into<Cow<'static, str>>) {
        *self = GoDescription::new(name);
    }

    /// Updates the GO description of the entity in place.
    ///
    /// This will allocate a new string if the description was previously
    /// created from a borrow.
    #[inline(always)]
    pub fn mutate<F: FnOnce(&mut String)>(&mut self, f: F) {
        f(self.description.to_mut());
    }

    /// Gets the look description of the entity as a `&str`.
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        &self.description
    }
}

impl std::fmt::Display for GoDescription {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.description, f)
    }
}

/* Conversions from strings */

impl From<&str> for GoDescription {
    #[inline(always)]
    fn from(name: &str) -> Self {
        GoDescription::new(name.to_owned())
    }
}
impl From<String> for GoDescription {
    #[inline(always)]
    fn from(name: String) -> Self {
        GoDescription::new(name)
    }
}

/* Conversions to strings */

impl AsRef<str> for GoDescription {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        &self.description
    }
}
impl From<&GoDescription> for String {
    #[inline(always)]
    fn from(val: &GoDescription) -> String {
        val.as_str().to_owned()
    }
}
impl From<GoDescription> for String {
    #[inline(always)]
    fn from(val: GoDescription) -> String {
        val.description.into_owned()
    }
}

impl Deref for GoDescription {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.description.as_ref()
    }
}
