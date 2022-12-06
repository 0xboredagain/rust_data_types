#![deny(clippy::all)]

fn main() {
    let first_name = "foo";
    let mut last_name = "bar";
    let age = 20u8;
    let population = 62_000_000;

    println!("hi {} {} of {} years of age", first_name, last_name, age);

    last_name = "baz";

    println!("population: {population}");
    println!("hi {} {} of {} years of age", first_name, last_name, age)
}

//data types
//types of data
