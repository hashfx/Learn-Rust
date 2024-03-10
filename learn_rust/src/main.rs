// allows unused variables that are declared but not used
#![allow(unused)]

// import libraries
use std::{cmp::Ordering, io};

fn main() {
    // anything with a "!"" is a Macro
    println!("What's your name?");
    
    // mutable variable name, by default all the are variables in rust are immutable
    let mut name = String::new();

    // take input from user + &mut is reference to variable that allows to save value directly to name
    // read_line returns enumerated type result
    io::stdin().read_line(&mut name).expect("Naam toh batao apna!");

    println!("Hello {}", name.trim_end());  // trim_end() gets rid of new line

    /* constants in Rust */
    const ONE_MILLION :u32 = 1_000_000;  // const are capital
    const PI:f32 = 3.14159265358979323846264338327950288419;

    // in Rust, we can have variable with same name and different data types
    let mut age:f32 = 21.0;
    let age :&str = "21";
    let mut age:u32 = age.trim().parse().expect("Age wasn't assigned");  // error handling on the go
    age = age + 1;  // age is mutable
    print!("I am {} years old and I have ${} in account", age, ONE_MILLION);


    /* match statement */
    match age{
        1..=17 => println!("Yet a kid"),  // range form 1 to 17, including 17
        18 | 59 => println!("Just at boundary"),  // 18 OR 59
        99..=u32::MAX => println!("You are beyond boundary, mate!"),  // maximum range
        _ => println!("This is default case"),  // default, matches everything else
    };

    /* Ordering
        This refers to the Ordering enum defined in the std::cmp module.
        It represents the result of a comparison between two values.
        Compulsory to use Less, Equal and Greater
    */
    let total_teeth =  32;
    let adult_teeth = 32;
    match total_teeth.cmp(&adult_teeth) {
        Ordering::Equal => println!("You have 32 teeth!"),
        Ordering::Less => println!("You yet a kid!"),
        Ordering::Greater => println!("You are a Donkey!")
    };

    /* Arrays */
    let arr_1 = [0, 1, 2, 3, 4];
    println!("arr[1] {}\nLength of array: {}", arr_1[1], arr_1.len());

}
