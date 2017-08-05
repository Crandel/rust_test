#![allow(dead_code)]
extern crate test_lib;

use test_lib::prim::{primitives, conditionals};
use test_lib::{structs, enums};

fn main() {
    println!("{}", "+".repeat(10));
    primitives::primitives_test();
    // println!("{}", "-".repeat(10));
    conditionals::conditionals_test();
    // println!("{}", "*".repeat(10));
    // loops::loops_test();
    // strings::strings_test();
    // input::input_test();
    // sequences::seq_test();
    // funcs::func_test();
    structs::structs_test();
    enums::enums_test();
    println!("{}", "/".repeat(10));
}
