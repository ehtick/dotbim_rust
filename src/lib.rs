mod color;
mod vector;
mod rotation;
mod mesh;
mod element;

use uuid::Uuid;
use serde_json;
use std::fs;


/// Reads dotbim file from path and creates a serde_json::Value with it
pub fn read_dotbim_to_value(path: &str) -> serde_json::Value {
    let text = fs::read_to_string(path).expect("Cannot read a dotbim file to string. Please check if path is correct.");
    let json: serde_json::Value = serde_json::from_str(&text).expect("Cannot parse a dotbim file into a json. Please check if json structure is correct.");
    json
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guid_works() {
        let id = Uuid::new_v4();

        println!("{}", id);
    }

    #[test]
    fn test_read_dotbim_to_value() {
        let path = "./test_files/CubesWithFaceColorsAndWithout_WithFixedNormals.bim";
        let value = read_dotbim_to_value(path);
        println!("{}", value);
    }
}
