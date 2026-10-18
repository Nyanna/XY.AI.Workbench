//! Extensible axis and value identifiers.

use std::fmt;
use std::sync::Arc;

macro_rules! string_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(Arc<str>);

        impl $name {
            pub fn new(id: impl AsRef<str>) -> Self {
                Self(Arc::from(id.as_ref()))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<&str> for $name {
            fn from(id: &str) -> Self {
                Self::new(id)
            }
        }

        impl From<String> for $name {
            fn from(id: String) -> Self {
                Self::new(id)
            }
        }

        impl From<&$name> for $name {
            fn from(id: &$name) -> Self {
                id.clone()
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!(stringify!($name), "({:?})"), &*self.0)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

string_id!(
    /// Capability axis such as `operands` or `scoring`; any string is valid.
    AxisId
);
string_id!(
    /// A value on an axis such as `fp16`; any string is valid.
    ValueId
);

impl ValueId {
    pub fn numeric(n: u32) -> Self {
        Self::new(n.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_compare_by_content() {
        assert_eq!(AxisId::new("operands"), AxisId::from("operands"));
        assert!(AxisId::new("a") < AxisId::new("b"));
        assert_eq!(ValueId::numeric(64).as_str(), "64");
    }

    #[test]
    fn display_is_plain() {
        assert_eq!(ValueId::new("fp16").to_string(), "fp16");
    }
}
