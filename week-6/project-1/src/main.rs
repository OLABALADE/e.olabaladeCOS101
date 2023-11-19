//A Rust program for the Student Council Voting System of Pan-Atlantic University.
use std::io;
//A function that ask for the user input and assign the input to a variable.
fn ask_for_input(que: &str) -> String {
    println!("{}", que);
    let mut var = String::new();
    io::stdin().read_line(&mut var).expect("Invalid Input");
    return var.trim().to_string().to_lowercase();
}
fn main() {
    let mut num_eligible = 0;

    while num_eligible < 150 {
        let name = ask_for_input("Your name");
        let state_of_origin = ask_for_input("Your State of origin");
        let department = ask_for_input("Your Department");
        let email = ask_for_input("Your Email");

        let class_rep = ask_for_input("Are you a class rep ?");

        let level = ask_for_input("What is your current Level");
        let level: f32 = level.parse().expect("Invalid Input");

        let cgpa = ask_for_input("What is your CGPA ?");
        let cgpa: f32 = cgpa.trim().parse().expect("Invalid Input");

        if class_rep == "yes" && cgpa > 4.0 && cgpa < 5.0 && level > 100.0 && level <= 500.0 {
            println!(
                " Name: {}\n State of origin: {}\n Department: {}\n Email: {}",
                name, state_of_origin, department, email
            );

            println!("You can vote!");
        } else {
            println!("Sorry, you are not eligible to vote");
        }
        num_eligible += 1;
    }
}
