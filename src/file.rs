use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use crate::element::Element;
use crate::mesh::Mesh;

/// Represents a file in the BIM format.
#[derive(Deserialize, Serialize)]
pub struct File {
    /// The schema version of the BIM file.
    schema_version: String,
    /// The list of meshes in the file.
    meshes: Vec<Mesh>,
    /// The list of elements in the file.
    elements: Vec<Element>,
    /// Additional information about the file.
    info: HashMap<String, String>
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        if self.schema_version != other.schema_version {
            return false;
        }
        if self.meshes.len() != other.meshes.len() {
            return false;
        }
        for i in 0..self.meshes.len() {
            if self.meshes[i] != other.meshes[i] {
                return false;
            }
        }
        if self.elements.len() != other.elements.len() {
            return false;
        }
        for i in 0..self.elements.len() {
            if self.elements[i] != other.elements[i] {
                return false;
            }
        }
        if !self.info.eq(&other.info) {
            return false;
        }
        true
    }
}

impl File {
    /// Returns a new File
    pub fn new(schema_version:String, meshes:Vec<Mesh>, elements:Vec<Element>, info:HashMap<String, String>) -> File { File {schema_version, meshes, elements, info} }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use serde_json::{from_str, from_value};
    use crate::color::Color;
    use crate::rotation::Rotation;
    use crate::vector::Vector;
    use super::*;

    fn get_file_with_triangle_blue_plate() -> File {
        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("Jane Doe"));

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Triangle"));

        File::new(
            String::from("1.0.0"),
            vec![
                Mesh::new(
                    0,
                    vec![
                        0.0,0.0,0.0,
                        10.0,0.0,0.0,
                        10.0,-15.0,0.0
                    ],
                    vec![
                        0,1,2
                    ]
                )
            ],
            vec![
                Element::new(
                    0,
                    Vector::new(0.,0.,0.),
                    Rotation::new(0.,0.,0.,1.0),
                    String::from("d4f28792-e1e9-4e31-bcee-740dbda61e20"),
                    String::from("Plate"),
                    Color::new(0,120,120,255),
                    None,
                    info
                )
            ],
            file_info
        )
    }

    #[test]
    fn test_new() {
        let result = get_file_with_triangle_blue_plate();

        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("Jane Doe"));

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Triangle"));

        let expected_mesh= Mesh::new(
            0,
            vec![
                0.0,0.0,0.0,
                10.0,0.0,0.0,
                10.0,-15.0,0.0
            ],
            vec![
                0,1,2
            ]
        );
        let expected_element= Element::new(
            0,
            Vector::new(0.,0.,0.),
            Rotation::new(0.,0.,0.,1.0),
            String::from("d4f28792-e1e9-4e31-bcee-740dbda61e20"),
            String::from("Plate"),
            Color::new(0,120,120,255),
            None,
            info
        );
        assert_eq!(String::from("1.0.0").eq(&result.schema_version), true);
        assert_eq!(result.meshes.len(), 1);
        assert_eq!(result.meshes[0].eq(&expected_mesh), true);
        assert_eq!(result.elements.len(), 1);
        assert_eq!(result.elements[0].eq(&expected_element), true);
        assert_eq!(file_info.eq(&result.info), true);
    }

    #[test]
    fn test_partial_eq_true() {
        let a = get_file_with_triangle_blue_plate();
        let b = get_file_with_triangle_blue_plate();
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partial_eq_different_schema_version_false() {
        let a = get_file_with_triangle_blue_plate();

        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("Jane Doe"));

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Triangle"));

        let b = File::new(
            String::from("Different"),
            vec![
                Mesh::new(
                    0,
                    vec![
                        0.0,0.0,0.0,
                        10.0,0.0,0.0,
                        10.0,-15.0,0.0
                    ],
                    vec![
                        0,1,2
                    ]
                )
            ],
            vec![
                Element::new(
                    0,
                    Vector::new(0.,0.,0.),
                    Rotation::new(0.,0.,0.,1.0),
                    String::from("d4f28792-e1e9-4e31-bcee-740dbda61e20"),
                    String::from("Plate"),
                    Color::new(0,120,120,255),
                    None,
                    info
                )
            ],
            file_info
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partial_eq_different_meshes_count_false() {
        let a = get_file_with_triangle_blue_plate();

        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("Jane Doe"));

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Triangle"));

        let b = File::new(
            String::from("1.0.0"),
            vec![
                Mesh::new(
                    0,
                    vec![
                        0.0,0.0,0.0,
                        10.0,0.0,0.0,
                        10.0,-15.0,0.0
                    ],
                    vec![
                        0,1,2
                    ],
                ),
                Mesh::new(
                    1,
                    vec![
                        0.0,0.0,0.0,
                        10.0,0.0,0.0,
                        10.0,-15.0,0.0
                    ],
                    vec![
                        0,1,2
                    ],
                )
            ],
            vec![
                Element::new(
                    0,
                    Vector::new(0.,0.,0.),
                    Rotation::new(0.,0.,0.,1.0),
                    String::from("d4f28792-e1e9-4e31-bcee-740dbda61e20"),
                    String::from("Plate"),
                    Color::new(0,120,120,255),
                    None,
                    info
                )
            ],
            file_info
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partial_eq_different_mesh_false() {
        let a = get_file_with_triangle_blue_plate();

        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("Jane Doe"));

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Triangle"));

        let b = File::new(
            String::from("1.0.0"),
            vec![
                Mesh::new(
                    0,
                    vec![
                        0.0,0.0,0.0,
                        10.0,0.0,0.0,
                        10.0,-15.0,0.0
                    ],
                    vec![
                        0,2,1 // Different
                    ]
                )
            ],
            vec![
                Element::new(
                    0,
                    Vector::new(0.,0.,0.),
                    Rotation::new(0.,0.,0.,1.0),
                    String::from("d4f28792-e1e9-4e31-bcee-740dbda61e20"),
                    String::from("Plate"),
                    Color::new(0,120,120,255),
                    None,
                    info
                )
            ],
            file_info
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partial_eq_different_elements_count_false() {
        let a = get_file_with_triangle_blue_plate();

        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("Jane Doe"));

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Triangle"));

        let b = File::new(
            String::from("1.0.0"),
            vec![
                Mesh::new(
                    0,
                    vec![
                        0.0,0.0,0.0,
                        10.0,0.0,0.0,
                        10.0,-15.0,0.0
                    ],
                    vec![
                        0,1,2
                    ]
                )
            ],
            vec![
                Element::new(
                    0,
                    Vector::new(0.,0.,0.),
                    Rotation::new(0.,0.,0.,1.0),
                    String::from("d4f28792-e1e9-4e31-bcee-740dbda61e20"),
                    String::from("Plate"),
                    Color::new(0,120,120,255),
                    None,
                    info.clone()
                ),
                Element::new(
                    0,
                    Vector::new(100.,0.,0.),
                    Rotation::new(0.,0.,0.,1.0),
                    String::from("882ccb70-9925-4a10-82af-07c6fa2be5e7"),
                    String::from("Plate"),
                    Color::new(0,120,120,255),
                    None,
                    info.clone()
                )
            ],
            file_info
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partial_eq_different_element_false() {
        let a = get_file_with_triangle_blue_plate();

        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("Jane Doe"));

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Triangle"));

        let b = File::new(
            String::from("1.0.0"),
            vec![
                Mesh::new(
                    0,
                    vec![
                        0.0,0.0,0.0,
                        10.0,0.0,0.0,
                        10.0,-15.0,0.0
                    ],
                    vec![
                        0,1,2
                    ]
                )
            ],
            vec![
                Element::new(
                    0,
                    Vector::new(0.,0.,0.),
                    Rotation::new(0.,0.,0.,1.0),
                    String::from("d4f28792-e1e9-4e31-bcee-740dbda61e20"),
                    String::from("Something else"), // Different
                    Color::new(0,120,120,255),
                    None,
                    info
                )
            ],
            file_info
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partial_eq_different_file_info_false() {
        let a = get_file_with_triangle_blue_plate();

        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("John Doe")); // different

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Triangle"));

        let b = File::new(
            String::from("1.0.0"),
            vec![
                Mesh::new(
                    0,
                    vec![
                        0.0,0.0,0.0,
                        10.0,0.0,0.0,
                        10.0,-15.0,0.0
                    ],
                    vec![
                        0,1,2
                    ]
                )
            ],
            vec![
                Element::new(
                    0,
                    Vector::new(0.,0.,0.),
                    Rotation::new(0.,0.,0.,1.0),
                    String::from("d4f28792-e1e9-4e31-bcee-740dbda61e20"),
                    String::from("Plate"),
                    Color::new(0,120,120,255),
                    None,
                    info
                )
            ],
            file_info
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_to_json() {
        let input = get_file_with_triangle_blue_plate();
        let input_serialized = to_string(&input);
        assert_eq!(input_serialized.is_ok(), true);
        let input_serialized_string = input_serialized.ok().unwrap();
        assert_eq!(input_serialized_string, "{\"schema_version\":\"1.0.0\",\"meshes\":[{\"mesh_id\":0,\"coordinates\":[0.0,0.0,0.0,10.0,0.0,0.0,10.0,-15.0,0.0],\"indices\":[0,1,2]}],\"elements\":[{\"mesh_id\":0,\"vector\":{\"x\":0.0,\"y\":0.0,\"z\":0.0},\"rotation\":{\"qx\":0.0,\"qy\":0.0,\"qz\":0.0,\"qw\":1.0},\"guid\":\"d4f28792-e1e9-4e31-bcee-740dbda61e20\",\"type\":\"Plate\",\"color\":{\"r\":0,\"g\":120,\"b\":120,\"a\":255},\"info\":{\"Name\":\"Triangle\"}}],\"info\":{\"Author\":\"Jane Doe\"}}");
    }

    #[test]
    fn test_from_json() {
        let json = "{\"schema_version\":\"1.0.0\",\"meshes\":[{\"mesh_id\":0,\"coordinates\":[0.0,0.0,0.0,10.0,0.0,0.0,10.0,-15.0,0.0],\"indices\":[0,1,2]}],\"elements\":[{\"mesh_id\":0,\"vector\":{\"x\":0.0,\"y\":0.0,\"z\":0.0},\"rotation\":{\"qx\":0.0,\"qy\":0.0,\"qz\":0.0,\"qw\":1.0},\"guid\":\"d4f28792-e1e9-4e31-bcee-740dbda61e20\",\"type\":\"Plate\",\"color\":{\"r\":0,\"g\":120,\"b\":120,\"a\":255},\"info\":{\"Name\":\"Triangle\"}}],\"info\":{\"Author\":\"Jane Doe\"}}";
        let actual_result = from_str::<File>(json);
        assert_eq!(actual_result.is_ok(), true);
        let actual = actual_result.ok().unwrap();
        let expected = get_file_with_triangle_blue_plate();
        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    fn test_create_and_read_pyramid() {
        let mesh = Mesh::new(
            0,
            vec![
                // Base
                0.0,0.0,0.0,
                10.0,0.0,0.0,
                10.0,10.0,0.0,
                0.0,10.0,0.0,

                // Top
                5.0,5.0,4.0
            ],
            vec![
                // Base faces
                0,1,2,
                0,2,3,

                // Side faces
                0,1,4,
                1,2,4,
                2,3,4,
                3,0,4
            ]
        );

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Pyramid"));

        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("John Doe"));
        file_info.insert(String::from("Date"), String::from("28.09.1999"));

        let element = Element::new(
            0,
            Vector::new(0.,0.,0.),
            Rotation::new(0., 0., 0., 1.),
            String::from("76e051c1-1bd7-44fc-8e2e-db2b64055068"),
            String::from("Structure"),
            Color::new(255,255,0,255),
            None,
            info,
        );

        let file: File = File::new(String::from("1.0.0"),
                             vec![mesh],
                             vec![element],
                             file_info);

        let file_serialized = to_string(&file);
        assert_eq!(file_serialized.is_ok(), true);

        let file_serialized_string = file_serialized.ok().unwrap();

        let path = "created_files/Pyramid.bim";

        fs::write(path, file_serialized_string).expect("Unable to write the file");

        let read_file = fs::File::open(path).expect("Cannot read the file");

        let json: serde_json::Value = serde_json::from_reader(read_file).expect("File has to be a proper JSON file");

        let read_file_unpacked: File = from_value(json).unwrap();

        assert_eq!(file.eq(&read_file_unpacked), true);
    }
}