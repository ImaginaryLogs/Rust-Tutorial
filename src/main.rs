#![allow(unused)] // only for a tutorial

use std::{i32, io};
// use std::io::* // if you want to include other libraries under this scope, place a *
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind}; // for files
use std::fs::File;
use std::cmp::Ordering; // for match!
 
fn example1_read_print_lines(){
    println!("What is your name?");
    let mut name = String::new();// Mutable means changable, immutable means not changable
    // Rust loves immutable since it eliminates having to track down how values change in your code.
    let greeting: &str = "Nice to meet you";

    // & is the pointer to name, just line in c.
    // read_line returns a result an enumarated type - which is a fixed type of possible values

    io::stdin().read_line(&mut name) 
        .expect("Didn't Received input"); // The values can be Ok and Error. Error tells you it fails and why it does fail.


    println!("Hello, world!"); // A macro is identified with the '!' at the end of the name of func.
    println!("Hello {}! {}", name.trim_end(), greeting);
}

fn example2_shadowing(){
    const ONE_MIL: u32 = 1_000_000; // u32 means unsigned integer with 32 bits, and _ allows to separate digits like comma for nums
    const PI: f32 = 3.14159265; // f32 is float 32
    let age: &str = "47";
    
    // You can redefine variables with the same name but different data types
    // This is called Shadowing.
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");

    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}

fn example3_max_size(){
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f32: {} ", f32::MAX);
    println!("Max f64: {} ", f64::MAX);
    // etc...

    let _is_true: bool = true; // the rust compiler will ignore any var starting with _ for any unused vars
    let my_grade = 'A'; // chars
    
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111); // only 6 digits of precision
    let num_1: f64 = 1.111111111111111;
    println!("f64: {}", num_1 + 0.111111111111111); // only 14 digits of precision

    let random_num: i32 = rand::thread_rng().gen_range(1..101); // includes to 1 and up to 100 (not including 101) 1 <= x < 101
    print!("Random : {}", random_num); 

}

fn example4_data_match() {
    let mut my_age = 47;

    let age: i32 = 8;
    if (age >= 1) && (age < 18) {
        println!("important birthday");
    } else if age >= 65 {
        println!("important birthday");
    } else if (age >= 1) || (age < 18) {
        println!("important birthday");
    } else {
        println!(":(");
    }

    let can_vote: bool = if my_age > 18 {
        true
    } else {
        false
    };
    println!("Can vote: {}", can_vote);

    let age2: i32 = 8;
    match age2 {
        1..=18 => println!("Important"),
        21 | 50 => println!("Important"),
        65..=i32::MAX => print!("important"),
        _ => print!("Not important") // you have to match every CASE and state
    }

    let my_age: i32 = 18;
    let voting_age: i32 = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    };
}

fn main() {
    // example1_read_print_lines();
    // example2_shadowing();
    // example3_max_size();
    // example4_data_match();
    

    
}
