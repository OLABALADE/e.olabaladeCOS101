struct Employee {
    ceo:String,
    company: String,
    age:u32
}

fn main() {
    //intialize a structure
    let emp1 = Employee {
        company:String::from("Microsoft Coporation"),
        ceo:String::from("Satya Nadella"),
        age:56
    };

    let emp2 = Employee {
        company:String::from("Google Inc."),
        ceo:String::from("Sundai Pichai"),
        age:51
    };
    //pass emp1 and emp1 to display()
    display(emp1);
    display(emp2);
}

//fetch values of specific structure fields using the
// operator and print it to the console

fn display(emp: Employee) {
    println!("Name is : {} comapany is {} age is
    {}", emp.ceo, emp.company, emp.age);
}
