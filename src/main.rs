#![feature(slice_concat_trait)]
use std::vec::Vec;
mod node;
mod process;
use node::node::Node;
use std::io;
use std::f64;

fn check_input(input: &str) -> bool {
    match input {
        "q" => ::std::process::exit(0),
        "y" => true,
         _ => false
    }
}

fn get_snow() -> f64 {
    println!("Now input amount of added snow");
    loop {
        let mut input = String::new();
        let result: Result<usize, std::io::Error> = io::stdin().read_line(&mut input);
        if result.is_ok() {
            let value_result = input.trim().parse::<f64>();
            if value_result.is_ok() {
                println!("Snow added");
                return  value_result.unwrap();
            } else {
                println!("Input figure");
            }
        }
    }
}

fn get_list(list: &mut Vec<Node>, snow: f64) {
    let mut input = String::new();
    println!("Input heights in figures. When finish write 'y'. To quit write 'q'");
    loop {
        input.clear();
        let result: Result<usize, std::io::Error> = io::stdin().read_line(&mut input);
        if result.is_err() {
            println!("Error {}", result.unwrap_err().to_string());
        } else {
            if check_input(&input.trim()) {
                break;
            }
            let value_result = input.trim().parse::<f64>();
            if value_result.is_err() {
                continue;
            }
            let value = value_result.unwrap();
            let node = Node {weight: value, pushed: snow};
            list.push(node);
            println!("Your input - {:?}", value);
        }
    }
}

fn main() {
    let snow: f64 = get_snow();
    let mut list: Vec<Node> = Vec::new();
    get_list(&mut list, snow);
    let formatted_list = list.iter()
        .map(|i| i.weight.to_string())
        .collect::<Vec<String>>().join(", ");
    println!("Total input - {}", formatted_list);
    println!("Snow is - {}", snow);
    process::process::process(&mut list);
}

