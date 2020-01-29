use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::Computer::ComputerIO;

fn read_file_content(filename: String) -> String {
  let mut f = File::open(filename).expect("file not found");
  let mut contents = String::new();

  f.read_to_string(&mut contents)
    .expect("Could not open file!");

  return contents;
}

mod Computer;

struct IO {
  value: i32,
}
impl ComputerIO for IO {
  fn get(&mut self) -> io::Result<i32> {
    Ok(self.value)
  }

  fn put(&mut self, value: i32) -> io::Result<()> {
    self.value = value;
    Ok(())
  }
}

fn main() {
  let filename = String::from("input.txt");
  let input: String = read_file_content(filename);

  let intcode: Vec<i32> = input
    .split(",")
    .into_iter()
    .map(|elem| elem.parse::<i32>().unwrap())
    .collect();

  println!("Num of intcode elements: {}", intcode.len());

  println!("Starting execution for part 1");
  let mut computer_part_1 = Computer::Computer::new(intcode.clone(), IO { value: 1 });
  computer_part_1.run();

  println!("Starting execution for part 2");
  // Part 2
  let mut computer_part_2 = Computer::Computer::new(intcode.clone(), IO { value: 5 });
  computer_part_2.run();
}
