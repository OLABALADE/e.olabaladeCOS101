//A Rust program for buying food at a restaurant
use std::io;
fn main() {
    println!(
        "
********************************************************
*                   Menu                   Price       *
*      P = Poundo Yam/Edinkaiko Soup       -N3,200     *
*      F = Fried Rice & Chicken            -N3,000     *
*      A = Amala & Ewedu Soup              -N2,500     *
*      E = Eba & Egusi Soup                -N2,000     *
*      W = White Rice & Stew               -N2,500     *
********************************************************
    "
    );
    //These variables will store the total price for each of the food on the menu.
    let mut p = 0.0;
    let mut f = 0.0;
    let mut a = 0.0;
    let mut e = 0.0;
    let mut w = 0.0;

    println!("Welcome, what will you be buying today ?.Pick p, f, a, e or w");
    'outer_loop: loop {
        let mut input1 = String::new();
        io::stdin()
            .read_line(&mut input1)
            .expect("Failed to read input");
        let input1 = input1.trim().to_lowercase();

        if input1 == "p" || input1 == "f" || input1 == "a" || input1 == "e" || input1 == "w" {

            println!("How many will you be buying ?");
            let mut num = String::new();
            io::stdin()
                .read_line(&mut num)
                .expect("Failed to read input");
            let num: f32 = num.trim().parse().expect("input is not a integer");

            if input1 == "p" {
                p += 3_200.00 * num;
            } else if input1 == "f" {
                f += 3_000.00 * num;
            } else if input1 == "a" {
                a += 2_500.00 * num;
            } else if input1 == "e" {
                e += 2_000.00 * num;
            } else if input1 == "w" {
                w += 2_500.00 * num;
            }
            /*The inner_loop gives the user the option of ordering more than one type of food 
            on the menu*/
            'inner_loop: loop {
                println!("Would you like to buy something else ?.Answer with 'yes' or 'no'");
                let mut input2 = String::new();
                io::stdin()
                    .read_line(&mut input2)
                    .expect("Failed to read input");
                let input2 = input2.trim().to_lowercase();

                if input2 == "yes" {
                    println!("What will you like buy ?");
                    continue 'outer_loop;
                } else if input2 == "no" {
                    break 'outer_loop;
                } else {
                    println!("Invalid input.Answer with 'yes' or 'no' ");
                    continue 'inner_loop;
                }
            }
        } else {
            println!("Invalid input.Pick p, f, a, e or w");
            continue;
        }
    }

    let mut total_cost = p + f + a + e + w;//This calculate the total cost
    /*The if statement checks if the user will be given 
    discount if the total cost is greater than N10,000*/
    if total_cost > 10_000.00 {
        total_cost = total_cost - (total_cost * 0.05);
    }
    println!("Total cost is N{}", total_cost);
}
