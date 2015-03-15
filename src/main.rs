extern crate rusco;
extern crate "toml" as toml;

use std::io::prelude::*;
use std::fs::File;

fn read_config() {

}

fn main() {

    let mut file = match File::open("Config.toml") {
        Err(e) => panic!("Error while reading config file: {}", e),
        Ok(file) => file
    };
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut parser = toml::Parser::new(&s);
    match parser.parse() {
        Some(value) => println!("found toml: {:?}", value),
        None => {
            println!("parse errors: {:?}", parser.errors);
        }
    }
}
