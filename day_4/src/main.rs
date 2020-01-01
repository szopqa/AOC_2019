#[derive(Debug)]
struct PasswordCracker {
  pass_lower_range: u32,
  pass_upper_limit: u32,
}
impl PasswordCracker {
  pub fn new(password_range: (u32, u32)) -> PasswordCracker {
    PasswordCracker {
      pass_lower_range: password_range.0,
      pass_upper_limit: password_range.1
    }
  }

  pub fn has_two_same_digits(possible_pass: &String) -> bool {
    let mut iteration = 0;
    let mut at_least_one = false;
    for each_char in possible_pass.chars() {
      iteration += 1;
      let next_elem = &possible_pass[iteration..if iteration == possible_pass.len() {iteration} else {iteration + 1}];

      if each_char.to_string() == next_elem {
        at_least_one = true;
        break;
      }
    }
    at_least_one
  }

  pub fn digits_dont_decrease(possible_pass: &String) -> bool {
    let mut iteration = 0;
    let mut digits_dont_decrease = false;
    for each_char in possible_pass.chars() {
      iteration += 1;
      if (iteration == possible_pass.len()) {
        break;
      }

      let next_elem = &possible_pass[iteration..iteration + 1];

      if each_char.to_digit(10).unwrap() > next_elem.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>() {
        digits_dont_decrease = false;
        break;
      } else {
        digits_dont_decrease = true;
      }
    }

    digits_dont_decrease 
  }

  pub fn crack(&self) -> i32 {
    let mut password_possibilities: Vec<String> = vec![];

    for each_possibility in self.pass_lower_range..self.pass_upper_limit {
      let each_possibility_str = each_possibility.to_string();

      if !Self::has_two_same_digits(&each_possibility_str) {
        continue;
      }

      if !Self::digits_dont_decrease(&each_possibility_str) {
        continue;
      }

      password_possibilities.push(each_possibility_str);
    }

    password_possibilities.len() as i32
  }
}

fn main() {
  // const puzzle_in: (u32, u32) = (114560, 114561);
  const puzzle_in: (u32, u32) = (109165, 576723);
  let mut pass_cracker = PasswordCracker::new(puzzle_in);

  let possible_pass_num = pass_cracker.crack();

  println!("There are {} password possibilities", possible_pass_num);
}
