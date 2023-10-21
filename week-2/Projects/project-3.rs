fn main() {

    let p:f64 = 210_000.00;// principal
    let r:f64 = 5.0;// rate
    let t:f64 = 3.0;// time

    let dep = p * (1.0 - (r / 100.0)).powf(t); //depreciation

    println!("Its value after three years is N{}", dep);

}