/*A Rust program that calculate the incentives of employees depending
 if they have any Work experience or not.*/
use std::io;
fn main() {
    loop {
        let mut is_experienced = String::new();
        println!("Do you have any Work experience.Choose yes or no");
        io::stdin()
            .read_line(&mut is_experienced)
            .expect("Failed to read input");
        let is_experienced = is_experienced.trim().to_lowercase();

        if is_experienced == "yes" {

            let mut age = String::new();
            println!("Enter your age");
            io::stdin()
                .read_line(&mut age)
                .expect("Failed to read input");
            let age: i32 = age.trim().parse().expect("Input is not a number");

            if age >= 40 {
                println!("Your incentive will be N1,560,000");
                break;
            } else if age >= 30 && age < 40 {
                println!("Your incentive will be N1,480,000");
                break;
            }
            if age < 28 {
                println!("Your incentive will be N1,300,000");
                break;
            }

        } else if is_experienced == "no" {
            println!("Your incentive will be N100,000");
            break;

        } else {
            println!("Invalid answer.Answer in 'yes' or 'no'");
            continue;/*This makes the user give the right input in the case he/she gave 
            the wrong input by repeating the loop.*/
        }
    }
}
