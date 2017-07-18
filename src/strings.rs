pub fn test_strings() {
    let strng = "Some simple string in the world";
    println!("Lenght: {}", strng.len());
    // Split using an index
    let (a, b) = strng.split_at(4);
    println!("{} - {}", b, a);
    // // create an iterator from string using char as delimeter
    // let mut str_count = strng.chars();
    // let mut i = str_count.next();
    // loop {
    //     match i {
    //         Some(x) => println!("{}", x),
    //         None => break,
    //     }
    //     i = str_count.next();
    // }

    // let mut str_count = strng.split_whitespace();
    // let mut i = str_count.next();
    // loop {
    //     match i {
    //         Some(x) => println!("{}", x),
    //         None => break,
    //     }
    //     i = str_count.next();
    // }

    // let strng2 = "First line\nSecond line\nthird line";
    // let mut str_count = strng2.lines();
    // let mut i = str_count.next();
    // loop {
    //     match i {
    //         Some(x) => println!("{}", x),
    //         None => break,
    //     }
    //     i = str_count.next();
    // }
    println!("find 'some' => {}", strng.contains("Some"));
}
