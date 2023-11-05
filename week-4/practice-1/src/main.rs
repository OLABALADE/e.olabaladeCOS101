fn main() {
    let _name = "Olabalade Emmanuel";
    let uni:&str = "Pan-Atlantic University";
    let addr: &str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("University: {}, \n Address: {}", uni, addr);

    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Science and Technology";
    println!("University: {}, \n School: {}", department, school);
}
