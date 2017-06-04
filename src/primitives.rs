// use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

// use std::io::stdin;

pub fn primitives() {
    println!("Hello, world!");

    let mut num: i64 = 34;
    println!("First num {}", num);
    num = 56;
    println!("Second num {}", num);
    // println!("Max i8 {}", i8::MAX);
    // println!("Min i8 {}", i8::MIN);
    // println!("Max i16 {}", i16::MAX);
    // println!("Min i16 {}", i16::MIN);
    // println!("Max i32 {}", i32::MAX);
    // println!("Min i32 {}", i32::MIN);
    // println!("Max i64 {}", i64::MAX);
    // println!("Min i64 {}", i64::MIN);
    // println!("Max u8 {}", u8::MAX);
    // println!("Min u8 {}", u8::MIN);
    // println!("Max u16 {}", u16::MAX);
    // println!("Min u16 {}", u16::MIN);
    // println!("Max u32 {}", u32::MAX);
    // println!("Min u32 {}", u32::MIN);
    // println!("Max u64 {}", u64::MAX);
    // println!("Min u64 {}", u64::MIN);
    // println!("Max isize {}", isize::MAX);
    // println!("Min isize {}", isize::MIN);
    // println!("Max usize {}", usize::MAX);
    // println!("Min usize {}", usize::MIN);
    // println!("Max f32 {}", f32::MAX);
    // println!("Min f32 {}", f32::MIN);
    // println!("Max f64 {}", f64::MAX);
    // println!("Min f64 {}", f64::MIN);

    trait ToString {
        fn to_string1(&self) -> String;
    }

    impl ToString for u32 {
        fn to_string1(&self) -> String {
            format!("String: {}", *self)
        }
    }

    let some_n: u32 = 45;
    let prn = some_n.to_string1();
    println!("{}", prn);
    let bool_var: bool = true;
    let some_char: char = 'd';
    println!("Bool: {0} and chars: {1}, it`s {0}", bool_var, some_char);
    println!("B: {:b}, H: {:x}, O: {:o}", 10, 10, 10);
    println!("{ten:>ws$}", ten = 10, ws = 4);
    println!("{ten:>0ws$}", ten = 10, ws = 6);
    println!("5 + 4 = {}", 5 + 4);
    println!("5 - 4 = {}", 5 - 4);
    println!("5 * 4 = {}", 5 * 4);
    println!("5.0 / 4.0 = {}", 5.0 / 4.0);
    println!("5.0 % 4.0 = {}", 5.0 % 4.0);

    let mut neg_4 = -4i32;

    println!("abs(-4) = {}", neg_4.abs());
    println!("4 ^ 6 = {}", 4i32.pow(6));
    println!("sqrt 9 = {}", 9f64.sqrt());
    println!("cbrt 9 = {}", 27f64.cbrt());
    println!("Round 1.45 = {}", 1.45f64.round());
    println!("Floor 1.45 = {}", 1.45f64.floor());
    println!("Ceiling 1.45 = {}", 1.45f64.ceil());
    println!("e ^ 2 = {}", 2f64.exp());
    println!("log(2) = {}", 2f64.ln());
    println!("log10(2) = {}", 2f64.log10());
    println!("90 to Radians = {}", 90f64.to_radians());
    println!("PI to Degrees = {}", 3.14f64.to_degrees());
    println!("Max 4, 5 = {}", 4f64.max(5f64));
    println!("Min 4, 5 = {}", 4f64.min(5f64));
}
