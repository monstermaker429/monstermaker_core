//! Definition of a monster.
//!
//! A [`Monster`](struct.Monster.html) is an instance of a defined
//! [`Species`](../species/struct.Species.html). Multiple 
//! [`Monster`](struct.Monster.html) objects may be defined to 
//! instantiate a single [`Species`](../species/struct.Species.html)
//! object. Unlike [`Species`](../species/struct.Species.html) objects,
//! [`Monster`](struct.Monster.html) objects are meant to be mutable.

use crate::species::Species;

/// An individual monster.
pub struct Monster {
    /// The [`Monster`](struct.Monster.html)'s name.
    pub name: &'static str,
    /// A reference to the [`Monster`](struct.Monster.html)'s Species.
    pub species: &'static Species,
}
