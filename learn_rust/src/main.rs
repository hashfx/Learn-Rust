// allows unused variables that are declared but not used
#![allow(unused)]

// import libraries
use std::io;

fn main() {
    // anything with a "!"" is a Macro
    println!("What's your name?");
    
    // mutable variable name, by default all the are variables in rust are immutable
    let mut name = String::new();

    // take input from user + &mut is reference to variable that allows to save value directly to name
    // read_line returns enumerated type result
    io::stdin().read_line(&mut name).expect("Naam toh batao apna!");

    println!("Hello {}", name.trim_end());  // trim_end() gets rid of new line
}
