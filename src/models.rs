use serde::{Deserialize, Serialize};

/// A simple representation of a fruit’s dimensions.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FruitDimensions {
    /// The fruit’s name (e.g. “Apple”).
    pub name: String,
    /// The length of the fruit in arbitrary units.
    pub length: f32,
    /// The width of the fruit in arbitrary units.
    pub width: f32,
    /// The height of the fruit in arbitrary units.
    pub height: f32,
}

impl FruitDimensions {
    /// Computes the approximate volume by multiplying length × width × height.
    pub fn volume(&self) -> f32 {
        self.length * self.width * self.height
    }
}
