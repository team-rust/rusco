extern crate rusco;
extern crate toml;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;

fn read_config() -> BTreeMap<String, toml::Value> {
    let mut file = File::open("Config.toml").unwrap();

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut parser = toml::Parser::new(&s);
    //parser.parse().unwrap();
    let config = match parser.parse() {
        None => panic!("test"),
        Some(config) => config,
     };
     config
}

fn main() {
    env_logger::init().unwrap();
    let config = read_config();
    info!("Config parsed: {:?}", config);

    for (k, v)  in config.iter() {
        println!("{} {:?}", k, v);
    }
}
