mod primitives;
mod conditionals;
mod loops;

fn main() {
    println!("{}", "+".repeat(10));
    primitives::primitives();
    println!("{}", "-".repeat(10));
    conditionals::cond();
    println!("{}", "*".repeat(10));
    loops::loops()
}
