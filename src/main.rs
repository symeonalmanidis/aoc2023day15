use indexmap::IndexMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("aoc2023day15.txt").unwrap();
    
    let hash = |string: &str| string
                                .chars()
                                .map(|c| c as usize)
                                .fold(0, |a, b| (a+b)*17%256 );

    //part1
    let sum1: usize = input.split(',').map(hash).sum();
    println!("{}", sum1);
    
    // part 2
    let mut boxes = Vec::new();
    boxes.resize(256, IndexMap::new());
    
    for operation in input.split(',') {
        if let Some((name, value)) = operation.split_once('=') {
            boxes[hash(name)].insert(name, value.parse::<usize>().unwrap());
        } else if let Some(name) = operation.strip_suffix("-") {
            boxes[hash(name)].shift_remove(name);
        } else {
            panic!("HOLY SHIT");
        }
    }
    
    let sum2: usize = boxes
        .iter()
        .enumerate()
        .flat_map(|(box_index, box_map)|
            box_map
                .iter()
                .enumerate()
                .map(move |(con_index, (_, value))| (box_index+1)*(con_index+1)*value)
                
        ).sum();
    println!("{}", sum2);
    
    
}