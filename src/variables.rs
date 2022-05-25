/*
 * Variables are `IMMUTABLE` by default - you cannont assign the variable to some other value after initializing it.
 * For a variable to point to something else (of the same type) later, put `mut` keyword before it
*/
fn main() {
    let mut target = "world";
    let mut greeting = "Hello";
    println!("{}, {}", greeting, target);

    greeting = "How are you doing";
    target = "mate";
    println!("{}, {}", greeting, target);
}
