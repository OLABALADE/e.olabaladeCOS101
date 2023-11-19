//A Rust program that prints a multiplication table from 1 to n.
use std::io;
fn main() {
    println!("Multiplication table from 1 to n.Input n");
    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Invalid Input");
    let n: i32 = n.trim().parse().expect("Not a valid integer");

    //Ask the user for the number of multiples for each table.
    println!("How many multiples do you need in each multiplication table ?");
    let mut m = String::new();

    io::stdin().read_line(&mut m).expect("Invalid Input");
    let m: i32 = m.trim().parse().expect("Not a valid integer");

    //i is the current multiplication table being computed.
    for i in 1..=n {
        // j is the current number between 1 and m that is being multiplied by i.
        for j in 1..=m {
            let x = i * j;
            println!("{} x {} = {}", i, j, x);
        }
    }
}
