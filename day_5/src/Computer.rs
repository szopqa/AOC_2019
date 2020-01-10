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
pub struct Computer {
  intcode: Vec<i32>,
  is_running: bool,
  current_intcode_pos: usize
}

impl Computer {
  pub fn new(intcode: Vec<i32>) -> Computer {
    Computer {
      intcode: intcode,
      is_running: true,
      current_intcode_pos: 0
    }
  }

  fn get_instruction(&self, position: usize) -> i32 {
    if position >= self.intcode.len() {
      return 0;
    }

    self.intcode[position]
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

  fn add(&mut self, parameters_modes: &Vec<ParameterMode>) -> () {
    self.current_intcode_pos += 4;
  }

  fn multiply(&mut self, parameters_modes: &Vec<ParameterMode>) -> () {
    self.current_intcode_pos += 4;
  }

  fn input(&mut self, parameters_modes: &Vec<ParameterMode>) -> () {
    self.current_intcode_pos += 2;
  }
  
  fn output(&mut self, parameters_modes: &Vec<ParameterMode>) -> () {
    self.current_intcode_pos += 2;
  }

  fn halt(&mut self) -> () {
    self.is_running = false;
  }

  fn calculate_next(&mut self) -> () {
    let (opcode, parameters_modes) = Self::decode_instruction(self.get_instruction(self.current_intcode_pos));

    println!("current opcode: {:?}", opcode);
    println!("current parameter modes: {:?}", parameters_modes);
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
    while (self.is_running) {
      self.calculate_next();
    }
  }
}