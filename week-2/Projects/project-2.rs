fn main() {
    /* 
    Total quantity sold.Each representing 
    Toshiba, Mac, Hp, Dell and 
    Acer respectively.
    */ 
    let total_qty:f64 = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;

    //Amount of each product
    let amt_toshiba:f64 = 450_000.00;
    let amt_mac:f64 = 1_500_000.00;
    let amt_hp:f64 = 750_000.00;
    let amt_dell:f64 = 2_850_000.00;
    let amt_acer:f64 = 250_000.00;

    // Sum of sales
    let total_amt = amt_toshiba + amt_mac + amt_hp + amt_dell + amt_acer;
    println!("The sum of sales is N{}", total_amt);

    // Average of sales
    let average = total_amt / total_qty;
    println!("The average is N{}", average);
    
}