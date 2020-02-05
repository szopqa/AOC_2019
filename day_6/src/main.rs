use std::fs::File;
use std::io;
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
struct Element {
    id: String
}

#[derive(Debug)]
struct Orbit {
    suborbits: HashMap<String, Vec<Orbit>>
}

struct Space {
    galaxy: Vec<Orbit>
}

impl Space {
    pub fn add_orbit_recursive(&mut self, root: Element, subelement: Element) -> () {
        println!("Adding {:?} as subelement of orbit {:?}", subelement, root);
        // TODO
    }
}

fn main() {
    let filename = String::from("test.txt");
    let input: String = read_file_content(filename);

    let mut space = Space{ galaxy: vec![] };

    for orbit_connection in input.lines() {
        let orbit_connection_parsed: Vec<&str> = orbit_connection.split(")").collect();
        let root: Element = Element { id: orbit_connection_parsed[0].to_string() };
        let subelement: Element = Element { id: orbit_connection_parsed[1].to_string() };
        space.add_orbit_recursive(root, subelement)
    }
}
