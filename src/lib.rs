use std::fs::File;
use std::io::{Read, BufReader};

pub mod tokenizer;

use crate::tokenizer::tokenizer;

pub fn run_jparser(path: &str){
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open file: {}", e);
            return;
        }
    };

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    match buf_reader.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => println!("Error Reading File: {:?}", e),
    }

    match tokenizer(&contents) {
        Ok(token) => println!("{:?}", token),
        Err(e) => println!("Error: {}", e)
    }

    
}