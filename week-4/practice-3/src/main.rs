fn main() {
    let _name1 = "Olabalade Emmanuel";
    println!("My name is {}", _name1);

    //find and replace 
    let _name2 = _name1.replace("Emmanuel","Chukwudi");
    println!("You can also call me {}", _name2);
    let faculty = "Faculty of Science and Technology";

    //find and replace
    let school = faculty.replace("Faculty", "School");
    println!("I am a student of {}", school);

}
