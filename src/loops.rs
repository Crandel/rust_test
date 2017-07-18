pub fn test_loops() {

    let mut x = 1;
    loop {
        if x % 2 == 0 {
            println!("infinite loop: x is {}", x);
        }
        if x == 15 {
            break;
        }
        x += 1;
    }
    println!("{}", "7".repeat(5));

    let mut y = 1;
    while y <= 15 {
        println!("while: y is {}", y);
        y += 1;
    }
    println!("{}", "c".repeat(5));

    for z in 1..10 {
        println!("For: z is {}", z);
    }
}
