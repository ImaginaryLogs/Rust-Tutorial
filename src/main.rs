#![allow(unused)] // only for a tutorial

use std::{char, i32, io, vec};
// use std::io::* // if you want to include other libraries under this scope, place a *
// use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind}; // for files
use std::fs::File;
use std::cmp::Ordering; // for match!
 
fn example01_read_print_lines(){
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

fn example02_shadowing(){
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

fn example03_max_size(){
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

fn example04_data_match() {
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

fn example05_arrays(){
    let arr_1 = [1, 2, 3, 4]; // automatically updates the type 
    println!("1st :  {}", arr_1[0]);
    println!("Length :  {}", arr_1.len());

    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9]; // automatically updates the type 
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    while loop_idx < arr_2.len() {
        println!("Arr : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    for val in arr_2.iter() {
        println!("Val : {}", val);
    }

}

fn example6_arrays() {
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);

    println!("Name : {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Age : {}", v1);
}

fn example7_string(){
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");

    for word in st1.split_whitespace(){
        println!("{}", word);
    }
 
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    let st3 = String::from("x r t b h k k a m c ");
    let mut v1: Vec<char> = st3.chars().collect(); // Vector
    v1.sort();
    v1.dedup(); // remove duplciates
    for char in v1 {
        println!("{}", char);
    }
    let st4: &str = "Random String";
    let mut st5: String = st4.to_string(); // stored in the heap
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6]; // grabes as slice from 0 to 5, not 6 inclusively
    println!("String length: {}", st6.len());
    st5.clear();
    let st6 = Stirng::from("Just some");
    let st7 = Stirng::from(" words");
    let st8 = st6 + &st7; // the values of st6 does not exist as it is now referenced in st8, but st7 does exist as we referenced those values.

    for char in st8.bytes(){
        println!("{}", char);
    } 
}

fn example8_casting(){
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32); 
}

fn example9_enumerations() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool{
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Day = Day::Monday;
    match today {
        Day::Monday => println!("Everyone hates Monday"),

        Day::Tuesday => println!("Donut day"),

        Day::Wednesday => println!("Hump day"),

        Day::Thursday => println!("Pay day"),

        Day::Friday => println!("Almost Weekend"),

        Day::Saturday => println!("Weekend"),

        Day::Sunday => println!("Weekend"),
    }

    println!("Is today the weekend {}", today.is_weekend());
}

fn example10_vectors(){
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    println!("1st : {}", vec2[0]);
    let second: &i32 = &vec2[1];

    match vec2.get(1) {
        Some(second ) => println!("2nd : {}", second),
        None => println!("No second value"),
    }

    for i in &mut vec2 {
        *i *= 2;
    }

    for i in &vec2 {
        println!("{}", i);
    }

    println!("Vec Length {}", vec2.len());
    println!("Pop: {:?}", vec2.pop());
}

fn example11_funcs(){
    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum of list = {}", sum_list(&num_list));
    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5.2 + 4.2 = {}", get_sum_gen(5.2, 4.2));
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for &val in list.iter() {
        sum += &val;
    }
    sum
}

fn sum_list(x:i32) -> (i32, i32) {
    return (x+1, x+2);
}


use std::ops::Add; // this will show the different data types that can be add with.

// we need an add trait
// T is a placeholder for a generic Type.
fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;   
}

use std::collections::HashMap;

// STACK : Stores values in LIFO format
// Data on the stack must have a defined fixed size.

// HEAP : When putting data on the heap you request a certain amount of space.
// The OS finds the space available and returns an address for that space by a pointer.


// BORROWING RULES
// 1. Each value has a variable that's called its owner. 
// 2. There is only one owner at a time. (No duplicate ownership)
// Example:
// let str1 = String::from("World");
// let str2 = st1 <--- ILLEGAL!
// 3. When the owner goes out of scope the value disappears.


// if compiler deems not necessary, it will be deleted in memory.   

fn change_string(name: &mut String) {
    name.push_str("is happy");
    println!("Message : {}", name);
}

fn example12_hash_map() {
    let mut heroes: HashMap<&str, &str> = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    for (k, v) in heroes.iter(){
        println!("{} = {}", k, v);
    }


    println!("Length : {}", heroes.len());

    if heroes.contains_key(&"Batman"){
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a Hero."),
            None => println!("Batman is not a Hero."),
        }
    }
}

fn example13_struct(){
    struct Customer {
        name: String, 
        address: String, 
        balance: f32
    }
    
    let mut bob: Customer = Customer{
        name: String::from("Bob Smith"),
        address:String::from("555 Main St."),
        balance: 234.50
    };

    bob.address = String::from("505 Main St");

    const PI: f32 = 3.14159265;
    trait Shape { // It is equivalent to interfaces in object oriented languages.
        fn new(length: f32, width: f32) -> Self;
        
        fn area(&self) -> f32; 
    }

    struct Rectangle {
        length: f32, width: f32
    }   ;
    struct Circle {
        length: f32, width: f32
    }

    impl Shape for Rectangle {
        fn new (length: f32, width: f32) -> Rectangle {
            return Rectangle{length, width};
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        } 
    }

    impl Shape for Circle {
        fn new (length: f32, width: f32) -> Circle {
            return Circle{length, width};
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        } 
    }
    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);
    println!("Red Area : {}", rec.area());
    println!("Circ Area : {}", circ.area());
}



fn main() {
    example01_read_print_lines();
    example02_shadowing();
    example03_max_size();
    example04_data_match();
    example05_arrays();
    example6_arrays();
    example7_string();
    example8_casting();
    example9_enumerations();
    example10_vectors();
    example11_funcs();
    example12_hash_map();
    example13_struct();
}
