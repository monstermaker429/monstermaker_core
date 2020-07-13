//! Definition of a species.
//!
//! The common practice is to define a [`Species`](struct.Species.html)
//! object and use immutable references to access its data from that 
//! point onward. [`Species`](struct.Species.html) objects are not 
//! traditionally intended to be mutated. Mutations instead are 
//! typically made on [`Monster`](../monster/struct.Monster.html) objects.

use crate::r#type::Type;

/// An individual species.
///
/// [`Species`](struct.Species.html) is simply a data store containing 
/// all the data defining the species.
pub struct Species<'a> {
    /// The [`Species`](struct.Species.html)' unique id.
    pub id: u16,
    /// The name of the [`Species`](struct.Species.html).
    pub name: String,
    
    /// A vector of the [`Species`](struct.Species.html)' 
    /// [`Types`](../type/struct.Type.html).
    pub types: Vec<&'a Type<'a>>,
    
    #[cfg(feature = "bestiary")]
    /// The species' category.
    pub category: String,
    #[cfg(feature = "bestiary")]
    /// A description of the species.
    pub description: String,
    #[cfg(feature = "bestiary")]
    /// The species' weight in hectograms.
    pub weight_in_hectograms: u16,
    #[cfg(feature = "bestiary")]
    /// The species' height in decimeters.
    pub height_in_decimeters: u16,
    
    // TODO: Continue expanding the features.
    /*
    base_hp: u8,
    base_attack: u8,
    base_defense: u8,
    base_special_attack: u8,
    base_special_defense: u8,
    base_speed: u8,
    hp_invariant: Option<u16>,
    attack_invariant: Option<u16>,
    defense_invariant: Option<u16>,
    special_attack_invariant: Option<u16>,
    special_defense_invariant: Option<u16>,
    speed_invariant: Option<u16>,
    hp_effort_value_yield: u8,
    attack_effort_value_yield: u8,
    defense_effort_value_yield: u8,
    special_attack_effort_value_yield: u8,
    special_defense_effort_value_yield: u8,
    speed_effort_value_yield: u8,
    
    base_experience: u16,
    base_friendship: u8,
    base_egg_cycles: u8,
    
    speed_performance_base: u8,
    speed_performance_max: u8,
    power_performance_base: u8,
    power_performance_max: u8,
    skill_performance_base: u8,
    skill_performance_max: u8,
    stamina_performance_base: u8,
    stamina_performance_max: u8,
    jump_performance_base: u8,
    jump_performance_max: u8,
    
    category: String,
    description: String,
    weight_in_hectograms: u16,
    height_in_decimeters: u16,
    color: &'a Color,
    shape: &'a Shape,
    habitat: &'a Habitat,
    */
}