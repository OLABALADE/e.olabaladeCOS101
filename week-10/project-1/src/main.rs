/* A Rust program that caculate the total cost supposing a
customer purchases 3 from each Laptop brand */

// A struct for the Laptops 
struct Laptop {
    cost: f32, // cost of the laptop
    amount: u32 // amount in stock
}

//Methods for the Laptop struct
impl Laptop {
    //This calculate the total cost for purchasing 3 laptops and returns it
    fn cost_of_3_laptops (&mut self) -> f32 {
        let total_cost = self.cost * 3.0;
        self.amount -= 3;
        total_cost
    }
}

fn main() {
    let mut hp = Laptop {
        cost: 650_000.00,
        amount: 10
    };

    let mut ibm = Laptop {
        cost: 755_000.00,
        amount: 6
    };

    let mut toshiba = Laptop {
        cost: 550_000.00,
        amount: 10
    };

    let mut dell = Laptop {
        cost: 850_000.00,
        amount: 4
    };

    let total_cost_of_12_laptops = hp.cost_of_3_laptops() + ibm.cost_of_3_laptops() +
        dell.cost_of_3_laptops() + toshiba.cost_of_3_laptops();

    println!("
    The total cost of purchasing three laptops from each brand is N{}", total_cost_of_12_laptops);
}
