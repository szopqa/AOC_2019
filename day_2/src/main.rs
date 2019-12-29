use std::fs::File;
use std::io::prelude::*;

fn read_file_content(filename: String) -> String {
  let mut f = File::open(filename).expect("file not found");
  let mut contents = String::new();

  f.read_to_string(&mut contents)
    .expect("Could not open file!");

  return contents;
}

mod Computer;

fn main() {
  let filename = String::from("input.txt");
  let input: String = read_file_content(filename);

  let single_integers: Vec<i32> = input
    .split(",")
    .into_iter()
    .map(|elem| elem.parse::<i32>().unwrap())
    .collect();

  println!("Num of intcode elements: {}", single_integers.len());

  // Part 1
  let intcode = Computer::Intcode::init(single_integers.clone(), 4);
  let mut computer = Computer::Computer::init_with_intcode(intcode);

  computer.setup_noun_and_verb(12, 2);
  let result_intcode = computer.calculate().unwrap();
  result_intcode.show();

  // Part 2
  let expected_val = 19690720;

  'outer: for noun in 0..100 {
    'inner: for verb in 0..100 {
      let experiment_intcode = Computer::Intcode::init(single_integers.clone(), 4);
      let mut experiment_computer = Computer::Computer::init_with_intcode(experiment_intcode);

      experiment_computer.setup_noun_and_verb(noun, verb);

      let experiment_results = experiment_computer.calculate().unwrap();

      if experiment_results.get_result() == expected_val {
        println!(
          "Found results for noun: {} and verb: {}. Result input: {}",
          noun,
          verb,
          100 * noun + verb
        );
        break 'outer;
      }
    }
  }
}
