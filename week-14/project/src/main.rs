//A Rust program that reads from SQL database files

use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    println!(
        "
    Welcome to Globalcom Ltd.
    Are you a/an :
    1) Administrator
    2) Project Manager
    3) Employee
    4) Customer
    5) Vendor
    "
    );

    // User input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: u32 = input.trim().parse().expect("Invalid input");

    // Matches the user input and prints the datafile linked to their input
    match input {
        1 => read_file("globacom_db.sql"),
        2 => read_file("project_db.sql"),
        3 => read_file("staff_db.sql"),
        4 => read_file("customer_db.sql"),
        5 => read_file("dataplan_db.sql"),
        _ => println!("Invalid Input"),
    }
}


// Reads and output the database file given the path parameter
fn read_file(path: &str) {
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("{}", content)
}
