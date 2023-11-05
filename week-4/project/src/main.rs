//Rust program that calculate the speed of a car in km/hr
use std::io;
fn main() {
    //accept_input is the outer loop that decide if the user want to find speed again.
    'accept_input: loop {
        println!("Enter distance in miles.");
        //distance in miles
        let mut distance = String::new();
        io::stdin()
            .read_line(&mut distance)
            .expect("Failed to read input");
        let distance: f32 = distance.trim().parse().expect("Not a valid number.");

        //time taken in hours
        println!("Enter time taken in hours.");

        let mut hour = String::new();
        io::stdin()
            .read_line(&mut hour)
            .expect("Failed to read input.");
        let hour: f32 = hour.trim().parse().expect("Not a valid number.");
        let speed = (distance * 0.621) / hour;

        println!("The car speed is {} km/hr.", speed);

        //answer_loop is the inner loop that runs again if the user gave a wrong input.
        'answer_loop: loop {
            println!("Do you want to calculate the speed again.Input Yes or No.");
            let mut answer = String::new();
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read answer");
            let answer = answer.trim().to_lowercase();/*to_lowercase solves problems
            casing by converting the input to lowercase.*/

            if answer == "yes" {
                continue 'accept_input;
            } else if answer == "no" {
                break 'accept_input;
            } else {
                println!("Invalid answer!.Please answer yes or no.");
                continue 'answer_loop;
            }
        }
    }
}
