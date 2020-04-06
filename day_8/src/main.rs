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

fn main() {
    let _filename: String = String::from("input.txt");
    let _input = read_file_content(_filename);
    let _elements = _input.len() as i32;

    let _pixels_in_layer: i32 = IMAGE_HIGHT * IMAGE_WIDTH;
    let _num_of_layers: i32 = _elements / _pixels_in_layer;
        
    let mut _layers: Vec<Layer> = vec![];
    for _layer_num in 0.._num_of_layers {
        let _lower_bound = (_layer_num * _num_of_layers) as usize;
        let _upper_bound = ((_layer_num + 1) * _num_of_layers - 1) as usize;

        _layers.push(Layer {
            _elements: &_input[_lower_bound.._upper_bound]
        })
    }
}