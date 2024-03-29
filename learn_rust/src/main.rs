// allows unused variables that are declared but not used
#![allow(unused)]

// import libraries
use std::{cmp::Ordering, io};
use std::ops::Add;  // generic trait
use std::collections::HashMap;  // hashmaps

fn main() {
    // anything with a "!"" is a Macro
    println!("What's your name?");

    // mutable variable name, by default all the are variables in rust are immutable
    let mut name = String::new();

    // take input from user + &mut is reference to variable that allows to save value directly to name
    // read_line returns enumerated type result
    io::stdin()
        .read_line(&mut name)
        .expect("Naam toh batao apna!");

    println!("Hello {}", name.trim_end()); // trim_end() gets rid of new line

    /* constants in Rust */
    const ONE_MILLION: u32 = 1_000_000; // const are capital
    const PI: f32 = 3.14159265358979323846264338327950288419;

    // in Rust, we can have variable with same name and different data types aka Shadowing
    let mut age: f32 = 21.0;
    let age: &str = "21";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned"); // error handling on the go
    age = age + 1; // age is mutable
    print!(
        "I am {} years old and I have ${} in account",
        age, ONE_MILLION
    );

    /* match statement */
    match age {
        1..=17 => println!("Yet a kid"), // range form 1 to 17, including 17
        18 | 59 => println!("Just at boundary"), // 18 OR 59
        99..=u32::MAX => println!("You are beyond boundary, mate!"), // maximum range
        _ => println!("This is default case"), // default, matches everything else
    };

    /* Ordering
        This refers to the Ordering enum defined in the std::cmp module.
        It represents the result of a comparison between two values.
        Compulsory to use Less, Equal and Greater
    */
    let total_teeth = 32;
    let adult_teeth = 32;
    match total_teeth.cmp(&adult_teeth) {
        Ordering::Equal => println!("You have 32 teeth!"),
        Ordering::Less => println!("You yet a kid!"),
        Ordering::Greater => println!("You are a Donkey!"),
    };

    /* Arrays */
    let arr_1 = [0, 1, 2, 3, 4];
    println!("arr[1] {}\nLength of array: {}", arr_1[1], arr_1.len());

    /* Loops */
    let mut loop_idx = 0; // index of loop, mutable so that it could be updated
    loop {
        // exit the loop if loop_idx exceeds the length of arr_1
        if loop_idx >= arr_1.len() {
            break;
        }
        // skip if element is not even
        if arr_1[loop_idx] % 2 != 0 {
            loop_idx += 1;
            continue;
        }
        println!("{}", arr_1[loop_idx]);
        loop_idx += 1;
    }

    /* while loop */
    let mut loop_idx = 0; // reset index to 0
    while loop_idx < arr_1.len() {
        println!("{}", arr_1[loop_idx]);
        loop_idx += 1;
    }

    /* for loop */
    for val in arr_1 {
        if arr_1[val] % 2 == 0 {
            println!("{}", val);
        }
    }

    /* tuple */
    let my_tuple: (u8, String, f64) = (69, "Hey".to_string(), 420.0);  // tuple of different data types
    println!("Message: {}", my_tuple.1);  // access elements via index

    // via separate variables
    let(v1, v2, v3) = my_tuple;
    println!("Age: {}\nNumber: {}", v1, v3);

    /* String:
    Strings can be of two types in Rust:
        String: vector of bytes that can be changed
        &str: points to string, allows viewing such strings
    */
    // growable string
    let mut str1 = String::new();
    str1.push('N');  // push a char
    str1.push_str(" word");  // push char at end
    println!("{}", str1);
    
    // separated by whitespace
    for word in str1.split_whitespace() {
        println!("{}", word);
    }

    // replace string
    let str2 = str1.replace('N', "B");
    println!("{}", str2);

    // create String and assign value at the same time
    let str3:String = String::from("a b c d e f G C d b a e");
    println!("{}", str3);

    // convert String to Vector
    let mut vec_str: Vec<char> = str3.chars().collect();

    // sort the char in vector
    vec_str.sort();

    // remove duplicates
    vec_str.dedup();

    for char in vec_str {
        println!("{}", char);  // output individual char
    }

    // String literal
    let str4: &str = "Yo, that's a Random String!";

    // convert str4 to a heap allocated string
    let mut str5: String = str4.to_string();
    println!("{}", str5);

    // convert string into array of bytes
    let byte_str_arr1 = str5.as_bytes();
    println!("Byte Str: {:?}", byte_str_arr1);

    // slice of a String
    let slice_str = &str5[0..9];  // 9 not included
    println!("String Slice: {}\nString Length: {}", slice_str, slice_str.len());

    // delete value from mutable String
    str5.clear();

    // combine Strings
    let str_a = String::from("1st String");
    let str_b = String::from("2nd String");
    let str_comb = str_a + &str_b;  // str_a doesn't exists coz it's in str_comb whereas str_b still exists
    println!("Combined String: {}", str_comb);

    /* Casting */
    let int_u8_1: u8 = 50;
    let int_u8_2: u8 = 100;
    let int_u32_3: u32 = (int_u8_1 as u32) + (int_u8_2 as u32);
    println!("u8: {}", int_u32_3);

    /* Enums */
    enum Day {
        Sunday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }

    let today:Day = Day::Wednesday;
    match today {
        Day::Wednesday => println!("It's Wednesday"),
        _ => println!("Not Wednesday!"),
    }
    println!("Is it weekend today {}", today.is_weekend());

    /* Vectors: stores values of same type, grows if mutable */
    
    // empty vector
    let empty_vector: Vec<i32> = Vec::new();

    // growable vector with defined values
    let mut grw_vec = vec![1, 2, 3, 4, 5];
    grw_vec.push(6);  // pust value to end
    println!("1st value: {}", grw_vec[0]);  // get value form index
    
    let second: &i32 = &grw_vec[1]; // verify if value exists
    match grw_vec.get(1) {
        Some(second) => println!("2nd value: {}", second),
        None => println!("No Second Value"),
    };

    // modify values in vector
    for i in &mut grw_vec {
        *i *= 2;
        println!("{}", i);
    }

    // length of vector
    println!("Length of Vector: {}", grw_vec.len());

    // remove last value
    println!("Pop: {:?}", grw_vec.pop());

    /* functions */

    println!("Addition: {}", get_sum(4, 5));
    
    let (val1, val2) = inc_dec(8);  // receive 2 values from single expression
    println!("Inc: {}, Dec: {}", val1, val2);

    let num_list = vec![5, 8, -8, -5];
    println!("Vector Sum: {}", vec_sum(&num_list));

    /* generics */
    println!("Generic sum 10.05 + 5.2 = {}", sum_generics(10.05, 5.2));
    println!("Generic sum 99 + (-1) = {}", sum_generics(99, -1));

    /*
    Ownership:
        Stack: Stores values in a last in first out format, data on the stack must have a defined fixed size
        Heap: When putting data on the heap you request a certain amount of space.
            The OS finds space available and returns an address for that space called a pointer.

        RULES in Rust:
            > Each value has a variable that's called its owner
            > There is only one owner at a time
            > When the owner goes out of scope the value disappears

    */

    // assignment vs clone operator in Ownership : works with string, arrays, vectors and not integers, bool, float or others
    let o_str_1: String = String::from("world");
    // let o_str_2: String = o_str_1;  // assign o_str_1 to o_str_2
    // println!("o_str_1 {}", o_str_1);  // error: borrow of moved value: `o_str_1`, value borrowed here after move
    let o_str_2: String = o_str_1.clone();  // creates 2 copies
    println!("Hello {}", o_str_1);


    /* hashmaps */
    let mut ipa = HashMap::new();
    
    // insert values to hashmap
    ipa.insert("A", "Alpha");
    ipa.insert("B", "Bravo");
    ipa.insert("C", "Charlie");
    ipa.insert("D", "Delta");

    // iterate over HashMap
    for(k, v) in ipa.iter(){
        println!("{}: {}", k, v);
    }

    // length of HashMap
    println!("Length of Hashmap: {}", ipa.len());

    // check for specific key index
    if ipa.contains_key("C") {
        let _charlie = ipa.get("C");

        match _charlie {
            Some(x) => println!("Charlie Charlie!"),
            None => println!("Charlie Negative"),
        }
    }

    /* struct */

    #[derive(Debug)]  // allows Rust to print its variants in a human-readable format
    enum Rank {Lt, Cpt, Mjr, Col}
    struct Soldier {
        name: String,
        rank: Rank,
        number: i32,
    }

    let mut karl = Soldier{name: String::from("Karl"), rank: Rank::Lt, number: 420};
    karl.rank = Rank::Cpt;
    // enums in Rust by default don't support assignment (=).
    // They are considered different variants of the same type,
    // and directly assigning a new variant isn't allowed.
    // println!("{:?}", karl.rank);  // FIXME: error [description above]
    karl = Soldier { name: karl.name.clone(), rank: Rank::Cpt, number: karl.number };
    println!("{:?}", karl.rank);

    // generic struct
    struct Rectangle<L, B>{
        length: L,
        breadth: B,
    }
    let square = Rectangle{length: 10, breadth: 10};

    /* trait */

    trait Shape {
        fn new (length: i32, width: i32) -> Self;
        fn area(&self) -> i32;
    }
    
    struct Rhombus {length: i32, width: i32};
    struct Kite {length: i32, width: i32};
    
    impl Shape for Rhombus {
        fn new(length: i32, width: i32) -> Rhombus {
            return Rhombus{length, width};
        }
        fn area(&self) -> i32 {
            return self.length * self.width;  // 0.5 * d1 * d2
        }
    }

    let rhombus: Rhombus = Shape::new(10, 12);
    println!("Area of Rhombus = {}", rhombus.area());

}

/* functions */

fn get_sum(x: i32, y: i32) -> i32 {
    x+y  // no semicolon because the statement doesn't evalute to a function
}

// return multiple values
fn inc_dec(a: i32) -> (i32, i32){
    return (a+1, a-1);
}

// receive vector as input
fn vec_sum(vec: &[i32]) -> i32 {
    let mut vec_sum = 0;
    for &val in vec.iter(){
        vec_sum += &val;
    }
    vec_sum  // return vac_sum
}

/* generics */

fn sum_generics<T: Add<Output=T>>(p: T, q: T) -> T {
    // after including generic add trait
    return p + q;
}