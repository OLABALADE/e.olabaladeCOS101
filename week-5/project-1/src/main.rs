//Rust program that calculate the roots of a quadratic equation given a, b and c.
use std::io;
fn main() {
    println!("Enter a");
    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Invalid string");
    let a:f32 = a.trim().parse().expect("Invalid number");

    println!("Enter b");
    let mut b = String::new();
    io::stdin()
        .read_line(&mut b)
        .expect("Invalid string");
    let b:f32 = b.trim().parse().expect("Invalid number");

    println!("Enter c");
    let mut c = String::new();
    io::stdin()
        .read_line(&mut c)
        .expect("Invalid string");
    let c:f32 = c.trim().parse().expect("Invalid number");

    let d:f32 = (-b).powf(2.0) - (4.0 * a * c);//This is the discriminant

    //This if statement checks if the discriminant is a negative or positive number.
    if d < -d  {
        println!("This equation has no real root.");

    } else {

        let d_sqrt = d.sqrt();
        let x:f32 = (-b + d_sqrt) / (2.0 * a);// first root
        let y:f32 = (-b - d_sqrt) / (2.0 * a);//second root

        println!("The roots of the equation are {} and {}", x, y);

    }
}
