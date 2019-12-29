use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Modules {
  total_fuel_part_one: u32,
  total_fuel_part_two: u32,
}

impl Modules {
  fn init() -> Modules {
    Modules {
      total_fuel_part_one: 0,
      total_fuel_part_two: 0,
    }
  }

  fn calc_fuel(mass: u32) -> u32 {
    let fuel: i32 = ((mass / 3) as i32) - 2;

    return if fuel <= 0 { 0 } else { fuel as u32 };
  }

  fn calc_for_part_one(&mut self, curr_mass: u32) -> () {
    let fuel_for_curr: u32 = Modules::calc_fuel(curr_mass);
    self.total_fuel_part_one += fuel_for_curr;
  }

  fn calc_for_part_two(&mut self, curr_mass: u32) -> () {
    let curr_fuel: u32 = Modules::calc_fuel(curr_mass);

    if curr_fuel > 0 {
      self.total_fuel_part_two += curr_fuel;

      Modules::calc_for_part_two(self, curr_fuel);
    }
  }

  fn get_total_fuel_part_one(&self) -> u32 {
    self.total_fuel_part_one
  }

  fn get_total_fuel_part_two(&self) -> u32 {
    self.total_fuel_part_two
  }
}

fn main() -> io::Result<()> {
  let filename = String::from("input.txt");
  let mut f = File::open(filename)?;
  let mut buffer = String::new();

  f.read_to_string(&mut buffer)?;

  let mut lines_iter: std::str::SplitWhitespace = buffer.split_whitespace();

  let mut modules = Modules::init();

  while let Some(line) = lines_iter.next() {
    let curr_mass: u32 = line.parse::<u32>().unwrap();

    // part one
    modules.calc_for_part_one(curr_mass);

    // part two
    modules.calc_for_part_two(curr_mass);
  }

  println!(
    "Total fuel required - part one: {}",
    modules.get_total_fuel_part_one()
  );
  println!(
    "Total fuel required - part two: {}",
    modules.get_total_fuel_part_two()
  );

  Ok(())
}
