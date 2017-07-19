pub fn test_func() {
    let (a, b) = (34, 65);
    let (a8, b8) = (3i8, 7i8);
    let s = sum_fn;
    println!("Sum of a({0}) + b({1}) is {2}", a, b, s(a, b));

    let closures_times = |x: i8, y: i8| x * y;
    let closures_div = || b8 / a8;
    println!("a8({0}) times b8({1}) is {2}", a8, b8, closures_times(a8, b8));
    println!("b8({1}) divaded by a8({0}) is {2}", a8, b8, closures_div());

    let vectr1 = vec![4, 6, 7, 8, 23];
    let v2 = vectr1;
    // error - couldn`t use it anymore
    // println!("{:?}", vectr1);
    println!("v2 -> {:?}", v2);
    let s = sum_vec(&v2);
    println!("Sum -> {}", s);
}

fn sum_fn(x: i32, y: i32) -> i32 {
    x + y
}

fn sum_vec(v1: &Vec<i32>) -> i32 {
    let sum = v1.iter().fold(0, |mut sum, &x| {sum += x; sum});
    return sum;
}
