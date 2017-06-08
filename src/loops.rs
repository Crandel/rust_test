pub fn loops() {
    let mut x = 1;
    loop {
        if ((x % 2) == 0) {
            println!("{}", x);
        }
        if x == 15 {
            break;
        }
        x += 1
    }
}
