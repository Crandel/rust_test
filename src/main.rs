#![allow(dead_code)]
// mod primitives;
// mod conditionals;
// mod loops;
// mod strings;
// mod input;
// mod sequences;
// mod funcs;
mod structs;
mod enums;

fn main() {
    println!("{}", "+".repeat(10));
    // primitives::primitives_test();
    // println!("{}", "-".repeat(10));
    // conditionals::conditionals_test();
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
