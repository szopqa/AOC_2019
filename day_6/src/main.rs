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

    /* 
     * orbitting element -> root element
     *
     * B->COM
     * C->B
     * D->C ...
     *
     * */
    pub fn calculate_checksum(&self) -> i64 {
        let mut _orbits = 0i64;

        let mut _steps_to_you = 0i64;
        let mut _steps_to_santa = 0i64;

        for (_orbitting_element, _root_element) in self.space_elements.iter() {
            let mut _parent = Some(_root_element);

            while let Some(_p) = _parent {
                 _orbits += 1;
                _parent = self.space_elements.get(_p);
            }
        }
        
        _orbits 
    }

    pub fn get_path_to<'a> (&'a self, _space_element: &'a String) -> Vec<&'a String> {
        let mut _path = vec![];

        let mut _destination = Some(_space_element);

        while let Some(_dest) = _destination {
            if _dest != _space_element {
                _path.push(_dest)
            }

           _destination = self.space_elements.get(_dest);
        }

        _path
    }
}

fn find_distance_between<'a> (path_a: Vec<&'a String>, path_b: Vec<&'a String>) -> usize {
    let mut path_between: Vec<&String> = vec![];

    for each_element in &path_a {
        if !path_b.contains(&each_element) {
            path_between.push(each_element);
        }
    }

    for each_element in &path_b {
        if !path_a.contains(&each_element) {
            path_between.push(each_element);
        }
    }

    path_between.len()
}

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
    
    // part 1
    println!("Found {} direct and indirect orbits", space.calculate_checksum());

    // part 2
    let _santa = String::from("SAN");
    let _you = String::from("YOU");

    let path_to_santa = space.get_path_to(&_santa);
    let path_to_you = space.get_path_to(&_you);

    let path_between = find_distance_between(path_to_santa, path_to_you);
    println!("Distance of {} moves is needed to reach santa", path_between);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_connections() {
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

    #[test]
    fn test_distance_between_you_and_santa() {
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
            "K)L",
            "K)YOU",
            "I)SAN"
        ];

        let mut space = Space::new();
        for orbit_connection in example_input.iter() {
            let orbit_connection_parsed: Vec<&str> = orbit_connection.split(")").collect();
            let _root = orbit_connection_parsed[0].to_string();
            let _subelement =  orbit_connection_parsed[1].to_string();
            space.add_new(_root, _subelement);
        }

        let _santa = String::from("SAN");
        let _you = String::from("YOU");

        let path_to_santa = space.get_path_to(&_santa);
        let path_to_you = space.get_path_to(&_you);

        assert_eq!(find_distance_between(path_to_santa, path_to_you), 4);
    }
}
