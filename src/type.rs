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
//! example, the following must be done to use the 
//! [`Type`](struct.Type.html) definition:
//!
//! ```
//! use monstermaker_core::r#type::Type;
//! ```

use hashable_rc::HashableWeak;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// A defined type.
///
/// [`Types`](struct.Type.html) interact with each other through their 
/// defined effectivenesses. [`Type`](struct.Type.html) effectiveness
/// is stored using 
/// [`Weak`](https://doc.rust-lang.org/std/rc/struct.Weak.html) 
/// references within a [`Type`](struct.Type.html) to other 
/// [`Types`](struct.Type.html) which are effective against it. For 
/// example:
///
/// ```
/// use monstermaker_core::r#type::Type;
/// use std::rc::Rc;
///
/// let foo = Rc::new(Type::new("foo".to_string()));
/// let bar = Rc::new(Type::new("bar".to_string()));
///
/// // Make bar have an effectiveness of 1.5 on foo.
/// foo.add_effectiveness(&bar, 1.5);
///
/// assert_eq!(foo.effectiveness_of_type(&bar), 1.5);
/// ```
///
/// Note that, since
/// [`Weak`](https://doc.rust-lang.org/std/rc/struct.Weak.html)
/// references are used internally, a strong reference counting (using
/// [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html)) must be
/// kept to preserve the reference.
/// [`Weak`](https://doc.rust-lang.org/std/rc/struct.Weak.html) 
/// references are used internally to avoid circular ownership, such as
/// in the case of a [`Type`](struct.Type.html) having effectiveness on 
/// itself.
///
/// ```
/// use monstermaker_core::r#type::Type;
/// use std::rc::Rc;
///
/// let foo = Rc::new(Type::new("foo".to_string()));
///
/// // Make foo have an effectiveness of 2.0 on itself.
/// foo.add_effectiveness(&foo, 2.0);
///
/// assert_eq!(foo.effectiveness_of_type(&foo), 2.0);
/// ```
pub struct Type {
    /// The name of the [`Type`](struct.Type.html).
    ///
    /// Note that [`Type`](struct.Type.html) objects are not identified
    /// by their `name` attributes. They are instead identified by 
    /// their references.
    pub name: String,
    
    // Access to effectivenesses is private to prevent direct access to
    // the mutable reference cell. Using a reference cell allows 
    // mutability of effectivenesses without having to worry about
    // maintaining mutability of the Types themselves.
    effectivenesses: RefCell<HashMap<HashableWeak<Type>, f32>>,
}

impl Type {
    /// Define a new [`Type`](struct.Type.html) object.
    ///
    /// The `name` provided here does not have to be unique.
    pub fn new(name: String) -> Type {
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
    /// for [`Weak`](https://doc.rust-lang.org/std/rc/struct.Weak.html) 
    /// references to be stored without worry of mutable borrow 
    /// checking.
    pub fn add_effectiveness(&self, other: &Rc<Type>, effectiveness: f32) {
        // TODO: consider marking as `unsafe`.
        self.effectivenesses.borrow_mut()
                            .insert(HashableWeak::new(Rc::downgrade(&other)), effectiveness);
    }
    
    /// Check the effectiveness of [`Type`](struct.Type.html) `other`
    /// on this [`Type`](struct.Type.html).
    ///
    /// If no effectiveness has been defined, a default value of `1.0`
    /// is returned.
    pub fn effectiveness_of_type(&self, other: &Rc<Type>) -> f32 {
        self.effectivenesses.borrow()
                            .get(&HashableWeak::new(Rc::downgrade(&other)))
                            .unwrap_or(&1.0)
                            .clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::r#type::Type;
    use std::rc::Rc;
    
    #[test]
    fn test_returns_effectiveness() {
        let type1 = Rc::new(Type::new("type1".to_string()));
        let type2 = Rc::new(Type::new("type2".to_string()));
        type1.add_effectiveness(&type2, 2.0);
        type2.add_effectiveness(&type1, 0.5);
        
        assert_eq!(type1.effectiveness_of_type(&type2), 2.0);
        assert_eq!(type2.effectiveness_of_type(&type1), 0.5);
    }
    
    #[test]
    fn test_no_effectiveness_returns_default() {
        let type1 = Rc::new(Type::new("type1".to_string()));
        let type2 = Rc::new(Type::new("type2".to_string()));
        
        assert_eq!(type1.effectiveness_of_type(&type2), 1.0);
    }
    
    #[test]
    fn test_effectiveness_of_own_type() {
        // Type objects should support containing weak references to 
        // themselves as effectiveness keys.
        let only_type = Rc::new(Type::new("only type".to_string()));
        only_type.add_effectiveness(&only_type, 2.0);
        
        assert_eq!(only_type.effectiveness_of_type(&only_type), 2.0);
    }
}
