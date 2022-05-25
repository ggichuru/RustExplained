use std::env;

fn main() {
    let name = env::args().skip(1).next();

    match name {
        Some(n) => println!("Hi {}", n),
        None => panic!("Didn't receive any name",),
    }
}

/* -----------------------------------------------------------
*              THE EXPLAINER
--------------------------------------------------------------*/
/*
* 1. Import a module called `env` from the `std` crate (libraries are called crates)
* 4. Call the function args() from the env module which returns an iterator(sequence) of arguments that have been passed to our program
            - Since the first arg contains our program name, we want to skip it.
            - skip(1) - skip 1 element
            - as iterators are lazy and don't precompute things, we have to ask it to give it the next element
            - `next()` returns an enum of type called `Option`
            - Option can either be a `Some(value)` or a `None` value.

* 6. We use rust `match` expression on the variable `name` and check it`s a `Some(n)` or a `None`.
* 7. when its a `Some(n)` we call `println!()`, passing our inner variable `n` thus greeting use.
* 8. when its None, we just `panic!()` - [macro that aborts the program, making it leave an error]
        < `println!()` >
            - accepts a string which can contain placeholder for items
            -> `{}` format specifier for printing simple types such as primitives
                    - calls a method form the `Display` trait
                    - used for dispalying human readable output of data types
            -> `{:?}` format specifier for printing other types
                    - calls a method from the `Debug` trait
                    - mostly used for debuging
*/
