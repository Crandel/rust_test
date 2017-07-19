pub fn test_func() {
    let (a, b) = (34, 65);
    let (a8, b8) = (3i8, 7i8);
    let s = sum_fn;
    println!("Sum of a({0}) + b({1}) is {2}", a, b, s(a, b));

    let closures_times = |x: i8, y: i8| x * y;
    let closures_div = || b8 / a8;
    println!("a8({0}) times b8({1}) is {2}", a8, b8, closures_times(a8, b8));
    println!("b8({1}) divaded by a8({0}) is {2}", a8, b8, closures_div());
}

fn sum_fn(x: i32, y: i32) -> i32 {
    x + y
}
