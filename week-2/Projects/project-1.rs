fn main() {
    let p: f64 = 520_000_000.00; // p is the principal
    let r: f64 = 10.00; // r is the rate
    let t: f64 = 5.0; // t represent time;

    let a = p * (1.0 + (r / 100.00)).powf(t); // a is the amount

    let ci = a - p; // ci is the compound interest
    println!("Compound interest is N{}", ci);
}
