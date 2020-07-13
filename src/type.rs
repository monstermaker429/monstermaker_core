//! Monster types and their interactions with each other.
//!
//! This module defines [`Type`](struct.Type.html) objects and handles
//! logic pertaining to their interactions with each other.
//!
//! Types may be added as attributes of many different objects in the
//! ecosystem. Each individual [`Type`](struct.Type.html) should only
//! be defined once, so that their mapped interactions remain 
//! consistent. Once defined, immutable references to the defined
//! [`Type`](struct.Type.html) objects should be used consistently.
//!
//! Note that you must escape this module's name to access it. For 
//! example, the following must be written to use the 
//! [`Type`](struct.Type.html) definition:
//!
//! ```
//! use monstermaker_core::r#type::Type;
//! ```

use std::cell::RefCell;
use std::collections::HashMap;

/// A defined type.
///
/// [`Types`](struct.Type.html) interact with each other through their 
/// defined effectivenesses. [`Type`](struct.Type.html) effectiveness
/// is stored using  the `name` attribute of other 
/// [`Types`](struct.Type.html) which are effective against it. For 
/// example:
///
/// ```
/// use monstermaker_core::r#type::Type;
///
/// let foo = Type::new("foo");
/// let bar = Type::new("bar");
///
/// // Make bar have an effectiveness of 1.5 on foo.
/// foo.add_effectiveness(&bar, 1.5);
///
/// assert_eq!(foo.effectiveness_of_type(&bar), 1.5);
/// ```
///
/// Additionally, [`Type`](struct.Type.html) objects can define 
/// effectiveness on themselves. To work around borrowing issues, 
/// adding effectiveness does not require mutability. This allows the
/// following example to work:
///
/// ```
/// use monstermaker_core::r#type::Type;
///
/// let foo = Type::new("foo");
///
/// // Make foo have an effectiveness of 2.0 on itself.
/// foo.add_effectiveness(&foo, 2.0);
///
/// assert_eq!(foo.effectiveness_of_type(&foo), 2.0);
/// ```
pub struct Type<'a> {
    /// The name of the [`Type`](struct.Type.html).
    ///
    /// Note that [`Type`](struct.Type.html) objects are not identified
    /// by their `name` attributes. They are instead identified by 
    /// their references.
    pub name: &'a str,
    
    // Access to effectivenesses is private to prevent direct access to
    // the mutable reference cell. Using a reference cell allows 
    // mutability of effectivenesses without having to worry about
    // maintaining mutability of the Types themselves.
    effectivenesses: RefCell<HashMap<&'a str, f32>>,
}

impl <'a> Type<'a> {
    /// Define a new [`Type`](struct.Type.html) object.
    ///
    /// The `name` provided here should be unique.
    pub fn new(name: &'a str) -> Type<'a> {
        Type {
            name: name,
            effectivenesses: RefCell::new(HashMap::new()),
        }
    }
    
    /// Define an `other` [`Type`](struct.Type.html) to have 
    /// effectiveness on this [`Type`](struct.Type.html) object.
    ///
    /// Note that, while this method does mutate the
    /// [`Type`](struct.Type.html) object, it does not require the
    /// [`Type`](struct.Type.html) object to be mutable. This allows 
    /// for effectiveness to be defined between a 
    /// [`Type`](struct.Type.html) object and itself.
    pub fn add_effectiveness(&self, other: &'a Type<'a>, effectiveness: f32) {
        self.effectivenesses.borrow_mut()
                            .insert(&other.name, effectiveness);
    }
    
    /// Check the effectiveness of [`Type`](struct.Type.html) `other`
    /// on this [`Type`](struct.Type.html).
    ///
    /// If no effectiveness has been defined, a default value of `1.0`
    /// is returned.
    pub fn effectiveness_of_type(&self, other: &Type) -> f32 {
        self.effectivenesses.borrow()
                            .get(&other.name)
                            .unwrap_or(&1.0)
                            .clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::r#type::Type;
    
    #[test]
    fn test_returns_effectiveness() {
        let type1 = Type::new("type1");
        let type2 = Type::new("type2");
        type1.add_effectiveness(&type2, 2.0);
        type2.add_effectiveness(&type1, 0.5);
        
        assert_eq!(type1.effectiveness_of_type(&type2), 2.0);
        assert_eq!(type2.effectiveness_of_type(&type1), 0.5);
    }
    
    #[test]
    fn test_no_effectiveness_returns_default() {
        let type1 = Type::new("type1");
        let type2 = Type::new("type2");
        
        assert_eq!(type1.effectiveness_of_type(&type2), 1.0);
    }
    
    #[test]
    fn test_effectiveness_of_own_type() {
        // Type objects should support assigning effectiveness on 
        // themselves.
        let only_type = Type::new("only type");
        only_type.add_effectiveness(&only_type, 2.0);
        
        assert_eq!(only_type.effectiveness_of_type(&only_type), 2.0);
    }
    
    #[test]
    fn test_duplicate_names() {
        // Defining two types with the same name.
        let type1 = Type::new("type1");
        let type2 = Type::new("type1");
        type1.add_effectiveness(&type1, 2.0);
        type1.add_effectiveness(&type2, 0.5);
        
        // The latter .add_effectiveness() call should overwrite the 
        // former.
        assert_ne!(type1.effectiveness_of_type(&type1), 2.0);
        assert_eq!(type1.effectiveness_of_type(&type2), 0.5);
    }
}
