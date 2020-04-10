use std::fs::File;
use std::io::prelude::*;

const IMAGE_WIDTH: i32 = 25;
const IMAGE_HIGHT: i32 = 6;

fn read_file_content(filename: String) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Could not open file!");

    return contents;
}

#[derive(Debug)]
struct Layer <'a> {
    _elements: &'a str
}

impl <'a> Layer <'a> { 
    fn get_num_of_occurrences(&self, _elem: char) -> i32 {
        let mut _out = 0;
        for _each_elem in self._elements.chars() {
            if _each_elem == _elem {
                _out += 1;
            }
        }
        _out
    }
}

fn main() {
    let _filename: String = String::from("input.txt");
    let _input = read_file_content(_filename);
    let _elements = _input.len() as i32;

    let _pixels_in_layer: i32 = IMAGE_HIGHT * IMAGE_WIDTH;
    let _num_of_layers: i32 = _elements / _pixels_in_layer;

    let mut _layers: Vec<Layer> = vec![];
    for _layer_num in 0.._num_of_layers {
        let _lower_bound = (_layer_num * _pixels_in_layer) as usize;
        let _upper_bound = ((_layer_num + 1) * _pixels_in_layer) as usize;

        _layers.push(Layer {
            _elements: &_input[_lower_bound.._upper_bound]
        })
    }

    let mut _min_zeros_occurrences_num = <i32>::max_value();
    let mut _layer_with_min_zeros: Layer = Layer{ _elements: "" }; 
    for _each_layer in _layers {
        let _min_zeros_occurrences_in_layer = _each_layer.get_num_of_occurrences('0');

        if _min_zeros_occurrences_in_layer < _min_zeros_occurrences_num {
            _layer_with_min_zeros = _each_layer;
            _min_zeros_occurrences_num = _min_zeros_occurrences_in_layer;
        }
    }

    let _part_one_result = _layer_with_min_zeros.get_num_of_occurrences('1') * _layer_with_min_zeros.get_num_of_occurrences('2');
    println!("Part one solution: {}", _part_one_result);
}
