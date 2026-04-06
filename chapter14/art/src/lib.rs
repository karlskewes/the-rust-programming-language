//! Art a library for modeling artistic concepts.

// Re-export sub-package items at top level.
pub use self::kinds::{PrimaryColor, SecondaryColor};
pub use self::utils::mix;

// Define "sub-package" items. Could be in different folders, nested, etc.
pub mod kinds {
    /// PrimaryColor per the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// SecondaryColor per the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// mix combines two PrimaryColor's to create a SecondaryColor
    pub fn mix(_c1: PrimaryColor, _c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Purple
    }
}
