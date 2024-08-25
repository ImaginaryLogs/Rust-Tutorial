#![allow(unused)] // only for a tutorial

use std::{char, i32, io, vec};
// use std::io::* // if you want to include other libraries under this scope, place a *
// use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind}; // for files
use std::fs::File;
use std::cmp::Ordering; // for match!


fn main(){
    example12_hash_map();
}