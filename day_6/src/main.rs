use std::fs::File;
use std::io;
use std::io::prelude::*;

fn read_file_content(filename: String) -> String {
  let mut f = File::open(filename).expect("file not found");
  let mut contents = String::new();

  f.read_to_string(&mut contents)
    .expect("Could not open file!");

  return contents;
}

#[derive(Debug)]
struct SpaceElement {
    element_name: String,
    element_path: Vec<String>
}

#[derive(Debug)]
struct Space {
    space_elements: Vec<SpaceElement>,
    orbit_count_checksum: i32
}

impl Space {
    pub fn new() -> Self {
        Self {
            space_elements: vec![],
            orbit_count_checksum: 0
        }
    }

    fn get_path_for_element(&self, element_name: &String) -> Option<&Vec<String>> {
        for each_space_elem in &self.space_elements {
            if (each_space_elem.element_name == *element_name) {
               return Some(&each_space_elem.element_path);
            }
        }
       None 
    }

    pub fn add_new(&mut self, root_element_name: String, orbiting_element: String) -> () {
        let path_search_res = self.get_path_for_element(&root_element_name);
        match path_search_res {
            Some(existing_path) => {
                println!("Existing path for {}: {:?}", root_element_name, existing_path);

                let mut new_elem_path = existing_path.to_vec().clone();
                new_elem_path.append(&mut vec![root_element_name]);

                //new element + length of already saved subelement connections
                let _num_of_connections: i32 = 1 + existing_path.to_vec().len() as i32;
                self.orbit_count_checksum += _num_of_connections;

                self.space_elements.push(
                    SpaceElement { 
                        element_name: orbiting_element,
                        element_path: new_elem_path 
                    }
                )
            },
            None => {
                println!("No path for {}. Creating new with {}",orbiting_element, root_element_name);
                self.orbit_count_checksum += 1;
                self.space_elements.push(
                    SpaceElement { 
                        element_name: orbiting_element,
                        element_path: vec![root_element_name]
                    }
                )
            }
        }
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
    let filename = String::from("test.txt");
    let input: String = read_file_content(filename);

    let mut space = Space::new();

    for orbit_connection in input.lines() {
        println!("Processing: {}", orbit_connection);
        let orbit_connection_parsed: Vec<&str> = orbit_connection.split(")").collect();
        let _root = orbit_connection_parsed[0].to_string();
        let _subelement =  orbit_connection_parsed[1].to_string();
        space.add_new(_root, _subelement);

//        println!("{:?}", space.space_elements);
    }

    println!("Found {} direct and indirect connections in current space map", space.orbit_count_checksum);
}
