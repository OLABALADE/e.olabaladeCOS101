//A Rust program that calculate the area of some shapes.
use std::io;

fn main() {
    println!("
    What shape area do you want to calculate (in metre square).
    Pick trapezium or rhombus or cube or cylinder or parallelogram");
    loop {
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read input");
        let answer = answer.trim().to_lowercase();

        if answer == "trapezium" {
            area_of_trapezium();
            break;

        } else if answer == "rhombus" {
            area_of_rhombus();
            break;

        } else if answer == "cube" {
            area_of_cube();
            break;

        } else if answer == "cylinder" {
            area_of_cylinder();
            break;

        } else if answer == "parallelogram" {
            area_of_parallelogram();
            break;

        //This run and redo the loop if the user gives an input value.    
        } else {
            println!("{} is not a valid input.Pick trapezium or rhombus or cube or cylinder or parallelogram", answer);
            continue;
        }
    }
}

//Asks the user a question,ask for input of number type and return a float number of  32 bits.
fn ask_for_input(que: &str) -> f32 {
    println!("{}", que);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input: f32 = input.trim().parse().expect("Input is not a number");
    input
}

//Calculates the area of a trapezium
fn area_of_trapezium() {
    let height = ask_for_input("What is its height");
    let base1 = ask_for_input("What is its first base length");
    let base2 = ask_for_input("What is its second base length");

    let area: f32 = (height * (base1 + base2)) / 0.5;
    println!("Area of trapezium is {}", area);
}

//Calculates the area of a rhombus
fn area_of_rhombus() {
    let diagonal1 = ask_for_input("What is its first diagonal length");
    let diagonal2 = ask_for_input("What is its second diagonal length");

    let area: f32 = (diagonal1 + diagonal2) / 0.5;
    println!("Area of rhombus is {}", area);
}

//Calculates the area of a parallelogram.
fn area_of_parallelogram() {
    let altitude = ask_for_input("What is its altitude");
    let base = ask_for_input("What is its base length");

    let area: f32 = altitude * base;
    println!("Area of parallelogram is {}", area);
}

//Calculates the area of a cube.
fn area_of_cube() {
    let length_of_side = ask_for_input("What is the length of its side");

    let area: f32 = 6.0 * (length_of_side).powf(2.0);
    println!("Area of cube is {}", area);
}

//Calculates the area of a cylinder
fn area_of_cylinder() {
    let height = ask_for_input("What is its height");
    let radius = ask_for_input("What is its radius");

    let area: f32 = 3.14 * radius.powf(2.0) * height;
    println!("Area of cyclinder is {}", area);
}
