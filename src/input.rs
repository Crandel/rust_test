use std::io::stdin;

pub fn input_test() {
    'outer: loop {
        let number: i32 = 15;
        println!("Pick a number!");

        loop {
            let mut line = String::new();

            let input = stdin().read_line(&mut line);

            let guess: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());

            match guess {
                None => println!("Enter number please!"),
                Some(n) if n == number => {
                    println!("You guessed a number, Congrats");
                    break 'outer
                },
                Some(n) if n < number => println!("Please enter bigger number"),
                Some(n) if n > number => println!("Please enter smaller number"),
                Some(_) => println!("Error!"),
            }
        }
    }
}
