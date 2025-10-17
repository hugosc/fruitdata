// ============================================================================
// models.rs
// ============================================================================
// This module defines the core data structures used by the fruitdata CLI.
// Specifically, it defines the `FruitDimensions` struct and implements the
// `volume()` method to calculate the volume of a fruit.
// ============================================================================

use serde::{Deserialize, Serialize};

/// A struct that represents a single fruit's dimensions and metadata.
///
/// The `#[derive(...)]` attributes below tell Rust to automatically generate
/// implementations for these traits:
///
/// - `Serialize`: Allows this struct to be converted to JSON using serde_json.
///   This is needed when saving fruits to the JSON file.
///
/// - `Deserialize`: Allows this struct to be created from JSON data using serde_json.
///   This is needed when loading fruits from the JSON file.
///
/// - `Debug`: Allows printing the struct with `{:?}` for debugging purposes.
///
/// - `Clone`: Allows creating copies of FruitDimensions instances. Useful when
///   we need to pass data without moving ownership.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FruitDimensions {
    /// The name of the fruit (e.g., "Apple", "Orange", "Banana").
    /// This is used to uniquely identify fruits in the catalogue.
    /// Names are case-insensitive when matching (handled in main.rs).
    pub name: String,

    /// The length of the fruit in arbitrary units (typically centimeters).
    /// Used in volume calculations and displayed to the user.
    pub length: f32,

    /// The width of the fruit in arbitrary units (typically centimeters).
    /// Used in volume calculations and displayed to the user.
    pub width: f32,

    /// The height of the fruit in arbitrary units (typically centimeters).
    /// Used in volume calculations and displayed to the user.
    pub height: f32,
}

impl FruitDimensions {
    /// Calculates the approximate volume of the fruit.
    ///
    /// This method computes the volume by multiplying all three dimensions:
    /// Volume = length × width × height
    ///
    /// This formula treats the fruit as a rectangular box, which is a simple
    /// approximation. In reality, fruits are irregular shapes, but this gives
    /// a rough estimate of size.
    ///
    /// # Returns
    /// An `f32` value representing the computed volume.
    ///
    /// # Example
    /// ```
    /// let apple = FruitDimensions {
    ///     name: "Apple".to_string(),
    ///     length: 4.0,
    ///     width: 2.5,
    ///     height: 1.5,
    /// };
    /// assert_eq!(apple.volume(), 15.0); // 4.0 * 2.5 * 1.5 = 15.0
    /// ```
    pub fn volume(&self) -> f32 {
        self.length * self.width * self.height
    }
}
