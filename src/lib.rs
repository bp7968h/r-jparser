use std::fs::File;

pub mod reader;
pub mod token;
pub mod value;
pub mod parser;

use crate::parser::JsonParser;

pub fn run_jparser(path: &str){
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open file: {}", e);
            return;
        }
    };

    match JsonParser::parse(file) {
        Ok(result) => println!("{:?}", result),
        Err(()) => ()
    }
}