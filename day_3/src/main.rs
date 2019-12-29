use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Point {
  x_pos: i32,
  y_pos: i32,
  num_of_moves_to_reach: i32,
}

#[derive(Debug)]
pub enum Directions {
  RIGHT,
  LEFT,
  UP,
  DOWN,
}

#[derive(Debug)]
pub struct Wire { 
  points: Vec<Point>,
  current_move_num: i32
}
impl Wire {
  pub fn new(wire_moves: Vec<Move>) -> Wire {
    let current_move_num = 0;
    let mut wire = Wire{
      points: vec![
        Point {
          x_pos: 0, 
          y_pos: 0,
          num_of_moves_to_reach: current_move_num
        }
      ],
      current_move_num: current_move_num
    };

    for wire_move in wire_moves {
      Wire::add_next_elem(&mut wire, wire_move);
    }

    wire
  }

  /* 
    x_dir and y_dir values: 
      1 -> move right/up, 
     -1 -> move left/down, 
      0 -> don't move
    returns only destination point
  */
  fn add_point_with_path(&mut self, move_distance: i32, x_dir: i32, y_dir: i32) -> &Point {
    let mut new_point: Point;

    for current_move_it in 0..move_distance {
      self.current_move_num += 1;
      let last_point: &Point = &self.points[self.points.len() - 1];
      new_point = Point {
        x_pos: last_point.x_pos + x_dir,
        y_pos: last_point.y_pos + y_dir,
        num_of_moves_to_reach: self.current_move_num
      };

      self.points.push(new_point);
    }

    &self.points[self.points.len() - 1] // returning destination point
  }

  fn add_next_elem(&mut self, next_move: Move) -> () {
    match next_move.direction {
      Directions::RIGHT => self.add_point_with_path(next_move.value, 1, 0),
      Directions::LEFT => self.add_point_with_path(next_move.value, -1, 0), 
      Directions::UP => self.add_point_with_path(next_move.value, 0, 1),
      Directions::DOWN => self.add_point_with_path(next_move.value, 0, -1),
    };
  }

  pub fn get_num_of_moves_for_point(&self, coll_point: &Point) -> u32 {
    let found_point = self.points.iter().filter(
      |p| p.x_pos == coll_point.x_pos && p.y_pos == coll_point.y_pos
    ).next().unwrap();

    found_point.num_of_moves_to_reach as u32
  }
}

#[derive(Debug)]
pub struct Move {
  pub direction: Directions,
  pub value: i32,
}

#[derive(Debug)]
struct Movement_interpreter { }
impl Movement_interpreter {
  pub fn get_moves(wire_moves: &str) -> Vec<Move> {
    let moves: Vec<Move> = wire_moves
      .split(",")
      .map(|move_elem|
        Move {
          direction: match &move_elem[..1] {
            "R" => Directions::RIGHT,
            "L" => Directions::LEFT,
            "U" => Directions::UP,
            "D" => Directions::DOWN,
            _ => panic!("Unexpected direction value!")
          },
          value: move_elem[1..].parse::<i32>().unwrap()
        }
      )
      .collect();

    moves
  }
}

#[derive(Debug)]
struct Collision <'a> { wires: &'a [Wire] }
impl <'a> Collision <'a> {
  pub fn new(wires: &[Wire]) -> Collision {
    Collision {
      wires: wires
    }
  }

  fn detect_wires_collision (wire_a: &'a Wire, wire_b: &'a Wire) -> Vec<Point> {
    let mut collision_points: Vec<Point> = Vec::new();
    for each_point_a in &wire_a.points { // TODO: optimise using lines (pair of points) instead of points
      for each_point_b in &wire_b.points {
        if each_point_a.x_pos == each_point_b.x_pos && each_point_a.y_pos == each_point_b.y_pos {
          collision_points.push(*each_point_a);
        }
      }
    }

    collision_points
  }

  pub fn detect_on_wires(&self) -> Vec<Point> {
    let mut gathered_collision_points: Vec<Point> = vec![];

    for wire_num in 0..self.wires.len() - 1 {
      let source_wire = self.wires.get(wire_num).unwrap();
      for wire_to_compare in self.wires.get(wire_num + 1..).unwrap() {
        let mut collision_points = Self::detect_wires_collision(source_wire, wire_to_compare);
        gathered_collision_points.append(&mut collision_points);
      }
    }

    gathered_collision_points
  }
}

fn read_file_content(filename: String) -> String {
  let mut f = File::open(filename).expect("file not found");
  let mut contents = String::new();

  f.read_to_string(&mut contents).expect("Could not open file!");

  return contents;
}

fn get_shortest_distance(collision_pts: &Vec<Point>) -> u32 { 
  let mut min_distance: u32 = <u32>::max_value();
  let start_x_pos = 0;
  let start_y_pos = 0;

  for each_point in collision_pts {
    let distance: u32 = ((start_x_pos - each_point.x_pos).abs() + (start_y_pos - each_point.y_pos).abs()) as u32;
    println!("Distance for point: x:{} y:{} -> {}", each_point.x_pos, each_point.y_pos, distance); 

    if distance != 0 && distance < min_distance {
      min_distance = distance;
    }
  };

  min_distance
}

fn main() {
  let filename = String::from("input.txt");
  let input: String = read_file_content(filename);

  let mut wires: Vec<Wire> = vec![];
  for wire_commands in input.lines() {
    let wire_moves = Movement_interpreter::get_moves(&wire_commands);
    
    let wire: Wire = Wire::new(wire_moves);
    wires.push(wire);
  }

  let collision_pts: Vec<Point> = Collision::new(&wires).detect_on_wires();
  println!("Found collision points: {:?}", collision_pts);

  // part 1
  let shortest_dist = get_shortest_distance(&collision_pts);
  println!("Shortest distance: {}", shortest_dist);

  // part 2
  let mut shortest_num_of_moves: u32 = <u32>::max_value();
  for each_collision_point in &collision_pts {
    let mut sum_of_moves = 0;
    for each_wire in &mut wires {
      sum_of_moves += each_wire.get_num_of_moves_for_point(each_collision_point);
    }
    if sum_of_moves != 0 && sum_of_moves < shortest_num_of_moves {
      shortest_num_of_moves = sum_of_moves;
    }
  }
  println!("Shortest number of moves for collision point: {}", shortest_num_of_moves);
}
