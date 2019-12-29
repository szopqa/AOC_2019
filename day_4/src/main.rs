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
  pub fn crack() -> () {}
}

fn main() {
  const puzzle_in: (u32, u32) = (109165, 576723);
  let mut pass_cracker = PasswordCracker::new(puzzle_in);
}
