# dotbim_rust ![Crates.io Version](https://img.shields.io/crates/v/dotbim_rust)

## Description

Open-source Rust library for dotbim file format.

Read more about dotbim here: https://github.com/paireks/dotbim

dotbim's website: https://dotbim.net/

Here you can find small manual for developers regarding development of tools that will work with .bim file format: https://github.com/paireks/dotbim/blob/master/DeveloperTips.md

## Installation

https://crates.io/crates/dotbim_rust

Run the following command:

```text
cargo add dotbim_rust
```

## Examples

Generally you can check the unit-tests to see multiple examples. However below is one of them:

### Pyramid example

```rust
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

let file_serialized_string = file_serialized.ok().unwrap();

let path = "created_files/Pyramid.bim";

fs::write(path, file_serialized_string).expect("Unable to write the file");
```

Then you can check if file saved looks correct in one of the existing viewers or thanks to connectors.

It should be like that:

![image](https://user-images.githubusercontent.com/47977819/154712470-aa4b5b44-3e23-4306-8a53-46d37494a52d.png)

## Libraries used

- For json serialization it uses serde, serde_json: https://serde.rs/
- For guid creation it uses uuid: https://github.com/uuid-rs/uuid
