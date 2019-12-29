#[derive(Debug)]
pub struct Intcode {
  pub integers: Vec<i32>,
  pub instr_pointer: usize,
  pub pointer_position: usize,
}

impl Intcode {
  pub fn init(integers: Vec<i32>, instr_pointer: usize) -> Intcode {
    Intcode {
      integers: integers,
      instr_pointer: instr_pointer, // temp -> should be moved to some computer setup struct
      pointer_position: 0,
    }
  }

  fn set_value_at_pos(&mut self, new_val: i32, pos: usize) -> () {
    self.integers[pos] = new_val;
  }

  fn get_value_from_pos(&self, pos: usize) -> i32 {
    self.integers[pos]
  }

  pub fn show(&self) -> () {
    println!("Intcode: {:?}", self.integers);
  }

  pub fn get_result(&self) -> i32 {
    self.integers[0]
  }
}

impl Iterator for Intcode {
  type Item = (i32, i32, i32, i32);

  fn next(&mut self) -> Option<Self::Item> {
    let curr_index: usize = self.pointer_position;
    self.pointer_position += self.instr_pointer;

    // check if instruction params overflows batch
    if self.integers.len() - curr_index < self.instr_pointer {
      return None;
    }

    Some((
      self.integers[curr_index],
      self.integers[curr_index + 1],
      self.integers[curr_index + 2],
      self.integers[curr_index + 3],
    ))
  }
}

#[derive(Debug)]
pub struct Computer {
  pub intcode: Intcode,
}

impl Computer {
  pub fn init_with_intcode(intcode: Intcode) -> Computer {
    Computer { intcode: intcode }
  }

  fn add(&self, num_1_pos: usize, num_2_pos: usize) -> i32 {
    self.intcode.get_value_from_pos(num_1_pos) + self.intcode.get_value_from_pos(num_2_pos)
  }
  fn multiply(&self, num_1_pos: usize, num_2_pos: usize) -> i32 {
    self.intcode.get_value_from_pos(num_1_pos) * self.intcode.get_value_from_pos(num_2_pos)
  }

  pub fn setup_noun_and_verb(&mut self, noun: i32, verb: i32) -> () {
    self.intcode.set_value_at_pos(noun, 1);
    self.intcode.set_value_at_pos(verb, 2);
  }

  pub fn calculate(&mut self) -> Option<&mut Intcode> {
    while let Some((opcode, num_1_pos, num_2_pos, target_pos)) = self.intcode.next() {
      match opcode {
        1 => self.intcode.set_value_at_pos(
          self.add(num_1_pos as usize, num_2_pos as usize),
          target_pos as usize,
        ),
        2 => self.intcode.set_value_at_pos(
          self.multiply(num_1_pos as usize, num_2_pos as usize),
          target_pos as usize,
        ),
        99 => {
          break;
        }
        _ => {
          panic!("Unexpected opcode value");
        }
      }
    }
    Some(&mut self.intcode)
  }
}
