//! Monster types and their interactions with each other.
//!
//! This module defines [`Type`](struct.Type.html) objects and handles
//! logic pertaining to their interactions with each other.
//!
//! Types may be added as attributes of many different objects in the
//! ecosystem. Each individual [`Type`](struct.Type.html) should be 
//! defined statically to be used throughout the program.
//!
//! Note that you must escape this module's name to access it. For 
//! example, the following must be written to use the 
//! [`Type`](struct.Type.html) definition:
//!
//! ```
//! use monstermaker_core::r#type::Type;
//! ```

use phf::Map;

/// A defined type.
///
/// [`Type`](struct.Type.html) objects are usually defined statically.
///
/// [`Types`](struct.Type.html) interact with each other through their 
/// defined effectivenesses. [`Type`](struct.Type.html) effectiveness
/// is stored using  the `name` attribute of other 
/// [`Types`](struct.Type.html) which are effective against it. These 
/// are defined using a
/// [`phf::Map`](https://docs.rs/phf/0.8.0/phf/struct.Map.html).
///
/// ```
/// use monstermaker_core::r#type::Type;
/// use phf::phf_map;
///
/// static FOO: Type = Type {
///     name: "foo",
///     effectivenesses: phf_map! {
///         "bar" => 1.5,
///     },
/// };
/// static BAR: Type = Type {
///     name: "bar",
///     effectivenesses: phf_map! {},
/// };
///
/// assert_eq!(FOO.effectiveness_of_type(&BAR), 1.5);
/// ```
///
/// If an effectiveness relationship is not defined, the default value
/// is `1.0`.
///
/// ```
/// use monstermaker_core::r#type::Type;
/// use phf::phf_map;
///
/// static FOO: Type = Type {
///     name: "foo",
///     effectivenesses: phf_map! {},
/// };
/// static BAR: Type = Type {
///     name: "bar",
///     effectivenesses: phf_map! {},
/// };
///
/// // The value defaults to 1.0.
/// assert_eq!(FOO.effectiveness_of_type(&BAR), 1.0);
/// ```
///
/// Additionally, [`Type`](struct.Type.html) objects can define 
/// effectiveness on themselves.
///
/// ```
/// use monstermaker_core::r#type::Type;
/// use phf::phf_map;
///
/// static FOO: Type = Type {
///     name: "foo",
///     effectivenesses: phf_map! {
///         "foo" => 2.0,
///     },
/// };
///
/// assert_eq!(FOO.effectiveness_of_type(&FOO), 2.0);
/// ```
pub struct Type {
    /// The name of the [`Type`](struct.Type.html).
    ///
    /// Note that [`Type`](struct.Type.html) objects are not identified
    /// by their `name` attributes. They are instead identified by 
    /// their references.
    pub name: &'static str,
    
    /// Effectiveness of other [`Types`](struct.Type.html) on this 
    /// [`Type`](struct.Type.html).
    ///
    /// Keyed by `name`. Prefer accessing through the
    /// [`effectiveness_of_type()`](#method.effectiveness_of_type)
    /// method.
    pub effectivenesses: Map<&'static str, f32>,
}

impl Type {
    /// Check the effectiveness of [`Type`](struct.Type.html) `other`
    /// on this [`Type`](struct.Type.html).
    ///
    /// If no effectiveness has been defined, a default value of `1.0`
    /// is returned.
    pub fn effectiveness_of_type(&self, other: &Type) -> f32 {
        self.effectivenesses.get(&other.name)
                            .unwrap_or(&1.0)
                            .clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::r#type::Type;
    use phf::phf_map;
    
    #[test]
    fn test_returns_effectiveness() {
        static TYPE1: Type = Type {
            name: "type1",
            effectivenesses: phf_map! {
                "type2" => 2.0,
            },
        };
        static TYPE2: Type = Type {
            name: "type2",
            effectivenesses: phf_map! {
                "type1" => 0.5,
            },
        };
        
        assert_eq!(TYPE1.effectiveness_of_type(&TYPE2), 2.0);
        assert_eq!(TYPE2.effectiveness_of_type(&TYPE1), 0.5);
    }
    
    #[test]
    fn test_no_effectiveness_returns_default() {
        static TYPE1: Type = Type {
            name: "type1",
            effectivenesses: phf_map! {},
        };
        static TYPE2: Type = Type {
            name: "type2",
            effectivenesses: phf_map! {},
        };
        
        assert_eq!(TYPE1.effectiveness_of_type(&TYPE2), 1.0);
    }
    
    #[test]
    fn test_effectiveness_of_own_type() {
        // Type objects should support assigning effectiveness on 
        // themselves.
        static ONLY_TYPE: Type = Type {
            name: "only type",
            effectivenesses: phf_map! {
                "only type" => 2.0,
            },
        };
        
        assert_eq!(ONLY_TYPE.effectiveness_of_type(&ONLY_TYPE), 2.0);
    }
}
