use std::fs::File;
use std::io;
use std::io::prelude::*;
use itertools::Itertools;

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
    phase_setting: i64, // amplifier phase setting from 0-4
    input: i64, // amplifier input -> output from prev amplifier
    pub output: i64, // amplifier output
    num_of_input_requests: i64
}

impl ComputerIO for IO {
    fn new(phase_setting: i64, input: i64) -> IO {
        Self {
            phase_setting,
            input,
            output: 0,
            num_of_input_requests: 0
        }
    }

    fn get(&mut self) -> io::Result<i64> {
        if self.num_of_input_requests == 0 {
            self.num_of_input_requests += 1;
            Ok(self.phase_setting)
        } else {
            Ok(self.input)
        }
    }

    fn put(&mut self, output: i64) -> io::Result<()> {
        self.output = output;
        Ok(())
    }

    fn output(&self) -> Option<i64> {
        if self.output != 0 {
            Some(self.output)
        } else {
            None
        }
    }
}

fn perform_thrusters_amplification(
    intcode: &Vec<i64>,
    (_a_ps, _b_ps, _c_ps, _d_ps, _e_ps): (i64, i64, i64, i64, i64)
) -> i64 {

    // first amplifier A
    let _amplifier_a_io = IO::new(_a_ps, 0);
    let mut _amplifier_a = Computer::Computer::new(intcode.clone(), _amplifier_a_io);
    let _amplifier_a_output = _amplifier_a.run().unwrap();

    // second amplifier B
    let _amplifier_b_io = IO::new(_b_ps, _amplifier_a_output);
    let mut _amplifier_b = Computer::Computer::new(intcode.clone(), _amplifier_b_io);
    let _amplifier_b_output = _amplifier_b.run().unwrap();
    
    // third amplifier C
    let _amplifier_c_io = IO::new(_c_ps, _amplifier_b_output);
    let mut _amplifier_c = Computer::Computer::new(intcode.clone(), _amplifier_c_io);
    let _amplifier_c_output = _amplifier_c.run().unwrap();
    
    // fourth amplifier D
    let _amplifier_d_io = IO::new(_d_ps, _amplifier_c_output);
    let mut _amplifier_d = Computer::Computer::new(intcode.clone(), _amplifier_d_io);
    let _amplifier_d_output = _amplifier_d.run().unwrap();
    
    // fifth amplifier E
    let _amplifier_e_io = IO::new(_e_ps, _amplifier_d_output);
    let mut _amplifier_e = Computer::Computer::new(intcode.clone(), _amplifier_e_io);
    let _amplifier_e_output = _amplifier_e.run().unwrap();
    
    _amplifier_e_output
}


fn perform_amplification_with_feedback_loop(
    intcode: &Vec<i64>,
    (_a_ps, _b_ps, _c_ps, _d_ps, _e_ps): (i64, i64, i64, i64, i64)
) -> i64 {
    let mut _last_e_amplifier_output = 0; // E output -> A input
    let mut _system_iteration_num = 0;

    loop {
        _system_iteration_num += 1;
        println!("");
        println!("Starting {} th system iteration, A input: {}", _system_iteration_num, _last_e_amplifier_output);

        // first amplifier A
        let _amplifier_a_io = IO::new(_a_ps, _last_e_amplifier_output);
        let mut _amplifier_a = Computer::Computer::new(intcode.clone(), _amplifier_a_io);
        let _amplifier_a_output = _amplifier_a.run_till_output().unwrap();
        println!("_aaaaa: {}", _amplifier_a_output);
        if !_amplifier_a.is_processing() { 
            println!("Stopping system: amplifier A stopped processing");
            break; 
        }

        // second amplifier B
        let _amplifier_b_io = IO::new(_b_ps, _amplifier_a_output);
        let mut _amplifier_b = Computer::Computer::new(intcode.clone(), _amplifier_b_io);
        let _amplifier_b_output = _amplifier_b.run_till_output().unwrap();
        println!("_bbbbbbbb: {}", _amplifier_b_output);
        if !_amplifier_b.is_processing() { 
            println!("Stopping system: amplifier B stopped processing");
            break; 
        }
        
        // third amplifier C
        let _amplifier_c_io = IO::new(_c_ps, _amplifier_b_output);
        let mut _amplifier_c = Computer::Computer::new(intcode.clone(), _amplifier_c_io);
        let _amplifier_c_output = _amplifier_c.run_till_output().unwrap();
        println!("_cccccccccc: {}", _amplifier_a_output);
        if !_amplifier_c.is_processing() { 
            println!("Stopping system: amplifier C stopped processing");
            break; 
        }
        
        // fourth amplifier D
        let _amplifier_d_io = IO::new(_d_ps, _amplifier_c_output);
        let mut _amplifier_d = Computer::Computer::new(intcode.clone(), _amplifier_d_io);
        let _amplifier_d_output = _amplifier_d.run_till_output().unwrap();
        println!("_dddddddd: {}", _amplifier_a_output);
        if !_amplifier_d.is_processing() { 
            println!("Stopping system: amplifier D stopped processing");
            break; 
        }

        // fifth amplifier E
        let _amplifier_e_io = IO::new(_e_ps, _amplifier_d_output);
        let mut _amplifier_e = Computer::Computer::new(intcode.clone(), _amplifier_e_io);
        _last_e_amplifier_output = _amplifier_e.run_till_output().unwrap();
        println!("_eeeeee: {}", _last_e_amplifier_output);
        if !_amplifier_e.is_processing() { 
            println!("Stopping system: amplifier E stopped processing");
            break; 
        }
    }
    
    _last_e_amplifier_output
}


fn main() {
    let filename = String::from("input.txt");
    let input: String = read_file_content(filename);

    let intcode: Vec<i64> = input
        .trim()
        .split(",")
        .into_iter()
        .map(|elem| elem.parse::<i64>().unwrap())
        .collect();

   let _max_amplification_part_one = 
       (0..5)
           .permutations(5)
           .map(|p| perform_thrusters_amplification(&intcode, (p[0], p[1], p[2], p[3], p[4])))
           .max()
           .unwrap();

   println!("Max amplification with linear system: {}", _max_amplification_part_one);

    let _max_amplification_part_two =
        (5..10)
            .permutations(5)
            .map(|p| perform_amplification_with_feedback_loop(&intcode, (p[0], p[1], p[2], p[3], p[4])))
            .max()
            .unwrap();

    println!("Max amplification for system with feedback loop: {}", _max_amplification_part_two);
}
