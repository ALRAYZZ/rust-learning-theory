use crate::garden::vegetables::Asparagus;

// Example of multiple imports in a single line
use std::{cmp::Ordering, io};
use std::io::{self, Write};
// Bring all public items
use std::collections::*;

pub mod garden;



fn main() {
    let plant = Asparagus {};
    println!("I planted {:?}", plant);
}
