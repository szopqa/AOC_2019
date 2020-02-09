use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn read_file_content(filename: String) -> String {
  let mut f = File::open(filename).expect("file not found");
  let mut contents = String::new();

  f.read_to_string(&mut contents)
    .expect("Could not open file!");

  return contents;
}

#[derive(Debug)]
struct Space {
    space_elements: HashMap<String, String>,
}

impl Space {
    pub fn new() -> Self {
        Self {
            space_elements: HashMap::new()
        }
    }

    pub fn add_new(&mut self, root_element_name: String, orbiting_element: String) -> () {
        self.space_elements.insert(orbiting_element, root_element_name);
    }

    pub fn calculate_checksum(&self) -> i64 {
        let mut _orbits = 0i64;
        for root_element in self.space_elements.values() {

            let mut _parent = Some(root_element);

            while let Some(_p) = _parent {
                 _orbits += 1;
                _parent = self.space_elements.get(_p);
            }
        }
        
        _orbits 
    }
}

/* Transforms following input:
    OM)B
    B)C
    C)D
    D)E
    E)F
    B)G


    into following dependency tree:
    B -> OM
    C -> B -> OM
    D -> C -> B -> OM
    E -> D -> C -> B -> OM
    F -> E -> D -> C -> B -> OM
    G -> B -> OM
*/

fn main() {
    let filename = String::from("input.txt");
    let input: String = read_file_content(filename);

    let mut space = Space::new();

    for orbit_connection in input.lines() {
        let orbit_connection_parsed: Vec<&str> = orbit_connection.split(")").collect();
        let _root = orbit_connection_parsed[0].to_string();
        let _subelement =  orbit_connection_parsed[1].to_string();
        space.add_new(_root, _subelement);
    }

    println!("Found {} direct and indirect orbits", space.calculate_checksum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let example_input = vec![
            "COM)B",
            "B)C",
            "C)D",
            "D)E",
            "E)F",
            "B)G",
            "G)H",
            "D)I",
            "E)J",
            "J)K",
            "K)L"
        ];

        let mut space = Space::new();
        for orbit_connection in example_input.iter() {
            let orbit_connection_parsed: Vec<&str> = orbit_connection.split(")").collect();
            let _root = orbit_connection_parsed[0].to_string();
            let _subelement =  orbit_connection_parsed[1].to_string();
            space.add_new(_root, _subelement);
        }

        assert_eq!(space.calculate_checksum(), 42);
    }
}
