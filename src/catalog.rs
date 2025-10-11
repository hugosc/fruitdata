use crate::models::FruitDimensions;
use std::error::Error;
use std::fs;

/// Load the fruit catalogue from a JSON file at the given path.
/// Returns a Vec of FruitDimensions or an error if reading/parsing fails.
pub fn load_catalogue(path: &str) -> Result<Vec<FruitDimensions>, Box<dyn Error>> {
    let json = fs::read_to_string(path)?;
    let fruits = serde_json::from_str(&json)?;
    Ok(fruits)
}

/// Save the fruit catalogue as pretty-printed JSON to the specified path.
/// Returns () on success or an error if serialization/writing fails.
pub fn save_catalogue(fruits: &[FruitDimensions], path: &str) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(fruits)?;
    fs::write(path, json)?;
    Ok(())
}

/// Create a default catalogue of fruits for initialisation.
pub fn initialise_fruit_catalogue() -> Vec<FruitDimensions> {
    vec![
        FruitDimensions {
            name: "Orange".into(),
            length: 5.0,
            width: 3.0,
            height: 2.0,
        },
        FruitDimensions {
            name: "Apple".into(),
            length: 4.0,
            width: 2.5,
            height: 1.5,
        },
        FruitDimensions {
            name: "Banana".into(),
            length: 6.0,
            width: 3.5,
            height: 2.5,
        },
        FruitDimensions {
            name: "Pear".into(),
            length: 6.0,
            width: 3.5,
            height: 2.5,
        },
    ]
}
