use std::io;

pub trait ComputerIO {
  fn get(&mut self) -> io::Result<i32>;
  fn put(&mut self, value: i32) -> io::Result<()>;
}

#[derive(Debug)]
enum Opcode {
  Add,
  Multiply,
  Input,
  Output,
  Halt
}

impl From<i32> for Opcode {
  fn from(code: i32) -> Self {
    match code {
      1 => Opcode::Add,
      2 => Opcode::Multiply,
      3 => Opcode::Input,
      4 => Opcode::Output,
      99 => Opcode::Halt,
      _ => panic!("Invalid opcode!")
    }
  }
}

#[derive(Debug)]
enum ParameterMode {
  Position,
  Immediate,
}

impl From<i32> for ParameterMode {
  fn from(param_mode: i32) -> Self {
    match param_mode {
      0 => ParameterMode::Position,
      1 => ParameterMode::Immediate, 
      _ => panic!("Invalid parameter mode!")
    }
  }
}

#[derive(Debug)]
pub struct Computer<T>
where
  T: ComputerIO
{
  pub computerIO: T,
  intcode: Vec<i32>,
  is_running: bool,
  current_intcode_pos: usize
}

impl<T> Computer<T>
where
  T: ComputerIO 
{
  pub fn new(intcode: Vec<i32>, computerIO: T) -> Self {
    Self {
      intcode,
      computerIO,
      is_running: true,
      current_intcode_pos: 0
    }
  }

  fn get_instruction(&self) -> i32 {
    let position: usize = self.current_intcode_pos;

    if position >= self.intcode.len() {
      return 0;
    }

    self.intcode[position]
  }

  fn write_in_memory(&mut self, address: usize, value: i32) -> () {
    self.intcode[address] = value;
  }

  fn decode_instruction(instruction: i32) -> (Opcode, Vec<ParameterMode>) {
    let opcode = instruction % 100;
    let param_modes = vec![
        ((instruction / 100) % 10).into(),
        ((instruction / 1000) % 10).into(),
        ((instruction / 10000) % 10).into(),
    ];

    (opcode.into(), param_modes)
  }

  fn get_value_at_index(&self, index: usize, parameter_mode: &ParameterMode) -> i32 {
    match parameter_mode {
      ParameterMode::Position => self.intcode[self.intcode[index] as usize],
      ParameterMode::Immediate => self.intcode[index]
    }
  }

  fn add(&mut self, parameters_modes: &Vec<ParameterMode>) -> () {
    let x_1: i32 = self.get_value_at_index(self.current_intcode_pos + 1, &parameters_modes[0]);
    let x_2: i32 = self.get_value_at_index(self.current_intcode_pos + 2, &parameters_modes[1]);
    // destination is always in position mode -> taking its value in Immediate mode
    let dest_index = self.get_value_at_index(self.current_intcode_pos + 3, &ParameterMode::Immediate) as usize;

    let result = x_1 + x_2;

    // println!("Saving ADDITION result: {} of numbers: x1 = {} x2 = {} at index: {}", result, x_1, x_2, dest_index);

    self.write_in_memory(dest_index, result);

    self.current_intcode_pos += 4;
  }

  fn multiply(&mut self, parameters_modes: &Vec<ParameterMode>) -> () {
    let x_1: i32 = self.get_value_at_index(self.current_intcode_pos + 1, &parameters_modes[0]);
    let x_2: i32 = self.get_value_at_index(self.current_intcode_pos + 2, &parameters_modes[1]);
    // destination is always in position mode -> taking its value in Immediate mode
    let dest_index = self.get_value_at_index(self.current_intcode_pos + 3, &ParameterMode::Immediate) as usize;

    let result = x_1 * x_2;

    // println!("Saving MULTIPLICATION result: {} of numbers: x1 = {} x2 = {} at index: {}", result, x_1, x_2, dest_index);

    self.write_in_memory(dest_index, result);

    self.current_intcode_pos += 4;
  }

  fn input(&mut self, parameters_modes: &Vec<ParameterMode>) -> () {
    let dest_index = self.get_value_at_index(self.current_intcode_pos + 1, &ParameterMode::Immediate) as usize;
    if let Ok(value) = self.computerIO.get() {
      self.write_in_memory(dest_index, value);
      self.current_intcode_pos += 2;
    } else {
      self.is_running = false;
    }
  }
  
  fn output(&mut self, parameters_modes: &Vec<ParameterMode>) -> () {
    let value = self.get_value_at_index(self.current_intcode_pos + 1, &parameters_modes[0]); // output also accepts param modes
    println!("Test finished with status: {}", value);
    if self.computerIO.put(value).is_ok() {
      self.current_intcode_pos += 2;
    } else {
      self.is_running = false;
    }
  }

  fn halt(&mut self) -> () {
    println!("Encountered HALT operation. Stopping intcode execution");
    println!("Intcode: {:?}", self.intcode);
    self.is_running = false;
  }

  fn calculate_next(&mut self) -> () {
    let (opcode, parameters_modes) = Self::decode_instruction(self.get_instruction());

    match opcode {
      Opcode::Add => self.add(&parameters_modes),
      Opcode::Multiply => self.multiply(&parameters_modes),
      Opcode::Input => self.input(&parameters_modes),
      Opcode::Output => self.output(&parameters_modes),
      Opcode::Halt => self.halt(),
      _ => panic!("Unknown opcode value!")
    }
  }

  pub fn run(&mut self) -> () {
    while self.is_running {
      self.calculate_next();
    }
  }
}