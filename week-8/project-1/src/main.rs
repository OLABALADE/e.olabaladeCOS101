//A Rust Program that find the APS position of an individual 

use std::io;
fn main() {
    //This will store the user input
    let mut info: Vec<String> = Vec::new();

    //Positions that holds APS 1-2
    let aps_1_2 = [
        "intern",
        "",
        "paralegal",
        "placement"
     
    ];

    //Positions that holds APS 3-5
    let aps_3_5 = [
        "administrator",
        "research assistant",
        "junior associate",
        "classroom teacher"
       
    ];

    //Positions that holds APS 5-8
    let aps_5_8 = [
        "senior administrator",
        "phd candidate",
        "associate",
        "senior teacher"

    ];

    //Positions that holds EL1 8-10
    let el1_8_10 = [
        "office manager",
        "post-doc researcher",
        "senior associate 1-2",
        "leading teacher"
      
    ];

    //Positions that holds EL2 10-13
    let el2_10_13 = [
        "director",
        "senior lecturer",
        "senior associate 3-4",
        "deputy principal"
      
    ];

    //Positions that holds SES
    let ses = [
        "ceo",
        "dean",
        "partner",
        "principal"
      
    ];

    //This ask for the user occupation
    let occupation = ask_for_input(
        "Which is your occupation:
   Office Administrator, or Academic, or Lawyer, or Teacher",
    );

    //The following if else statements finds the individual level in their occupation
    if occupation == "office administrator" {
        let office_level = ask_for_input(
            "Are you a/an:
        Intern or,
        Administrator or,
        Senior Administrator or,
        Office Manager or,
        Director or,
        CEO",
        );
        info.push(office_level);

    } else if occupation == "academic" {
        let academic_level = ask_for_input(
            "Are are you a/an:
        Research Assistant or,
        PhD Candidate or,
        Post-Doc Researcher or,
        Senior Lecturer or,
        Dean",
        );
        info.push(academic_level);

    } else if occupation == "lawyer" {
        let law_level = ask_for_input(
            "Are are you a/an:
        Paralegal or,
        Junior Associate or,
        Associate or,
        Senior Associate 1 - 3 or,
        Senior Associate 3 - 5 or,
        Partner",
        );
        info.push(law_level);
        
    } else if occupation == "teacher" {
        let teaching_level = ask_for_input(
            "Are are you a/an:
        Placement or,
        Classroom Teacher or,
        Senior Teacher or,
        Leading Teacher or,
        Deputy Principal or,
        Principal",
        );
        info.push(teaching_level);
    }

    //User years of experience
    let years_experience = ask_for_input("How many years of experience do you have in your occupation");
    info.push(years_experience.clone());
    let experience:i32 = years_experience.parse().expect("Invalid number");


     //The following if else statements finds the User APS position
    if aps_1_2.contains(&info[0].as_str()) && experience >= 1 && experience < 3 {
        println!("You hold position APS 1-2");
   
    } else if aps_3_5.contains(&info[0].as_str()) && experience >= 3 && experience < 5 {
        println!("You hold position APS 3-5");

    } else if aps_5_8.contains(&info[0].as_str()) && experience >= 5 && experience < 8 {
        println!("You hold position APS 5-8");

    } else if el1_8_10.contains(&info[0].as_str()) && experience >= 8 && experience < 10 {
        println!("You hold position EL1 8-10");

    } else if el2_10_13.contains(&info[0].as_str()) && experience >= 10 && experience <= 13 {
        println!("You hold position EL2 10-13");

    } else if ses.contains(&info[0].as_str()) && experience > 13 {
        println!("You hold position SES");
    } else {
        println!("Could not determine the position you hold.Check your inputs for any mistakes.")
    }
}

//This ask for input and return the input as string
fn ask_for_input(que: &str) -> String {
    println!("{}", que);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim().to_lowercase();
    input
}
