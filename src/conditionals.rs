pub fn main() {
    let age = 6;
    let some = true;
    if age < 3 {
        println!("{} is lower 3", age);
    } else if (age >= 5) && (some) {
        println!("{0} && {1}", age, some);
    }
    println!("!true = {}", !some);
    let ternar = if (age >= 8) { true } else { false };
    println!("ternar is {}", ternar);
}
