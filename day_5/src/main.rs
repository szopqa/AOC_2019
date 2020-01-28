use std::fs::File;
use std::io::prelude::*;
use std::io;

use crate::{
  Computer::{ComputerIO}
};

fn read_file_content(filename: String) -> String {
  let mut f = File::open(filename).expect("file not found");
  let mut contents = String::new();

  f.read_to_string(&mut contents)
    .expect("Could not open file!");

  return contents;
}

mod Computer;

struct IO { value: i32 }
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

  // Part 1
  let mut computer = Computer::Computer::new(intcode, IO {value: 1});
  computer.run();
}