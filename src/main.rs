#![allow(dead_code)]
extern crate test_lib;

// use test_lib::prim::{conditionals, primitives};
use test_lib::*;

use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut err = io::stderr();

    println!("{:?}", args);
    println!("{}", "+".repeat(10));
    let base = base_test::Basic::new(String::from("admin"), String::from("pass"));
    let base_str = base.encode_tostr();
    println!("{}", base_str);
    // primitives::primitives_test();
    // println!("{}", "-".repeat(10));
    // conditionals::conditionals_test();
    // println!("{}", "*".repeat(10));
    // loops::loops_test();
    // strings::strings_test();
    // input::input_test();
    // sequences::seq_test();
    // funcs::func_test();
    // structs::structs_test();
    // enums::enums_test();
    println!("{}", "/".repeat(10));
}
