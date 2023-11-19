/*A Rust program that calculate the incentives of researchers of the
Nigeria Reasearchers Guide(NGR).*/

use std::io;

fn main() {
    let num_of_researchers = 0;
    //This program will run only for 500 researchers.
    while num_of_researchers < 500 {
        println!("Welcome , What is your name ?");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Invalid input");
        let name = name.trim();

        println!("How many paper have you published ?");
        let mut num_of_paper = String::new();
        io::stdin()
            .read_line(&mut num_of_paper)
            .expect("Invalid input");
        let num_of_paper: i32 = num_of_paper.trim().parse().expect("Not a valid number");

        if num_of_paper >= 3 && num_of_paper <= 5 {
            println!("{}, your incentive is N500,000", name);
        }
        if num_of_paper > 5 && num_of_paper < 10 {
            println!("{}, your incentive is N800,000", name)
        }
        if num_of_paper >= 10 {
            println!("{}, your incentive is N1,000,000", name)
        }
        if num_of_paper < 3 {
            println!("{}, your incentive is N100,000", name)
        }
    }
}
