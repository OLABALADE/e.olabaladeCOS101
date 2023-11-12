fn main() {
    let mut count = 0;
    for num in 1..21 {
        if num > 10 {
            println!("{:?}", num);
            count += 1;
            continue;
        }
        
    }
    println!("Count values of values greater than 10 (between 1 and 20) is : {}", count);
}
