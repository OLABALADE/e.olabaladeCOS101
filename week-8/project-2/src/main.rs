//A Rust that finds the developer with the most years of experience in an interview
use std::io;

fn main() {
    //This will store all the developers information
    let mut all_developers_info: Vec<Vec<String>> = Vec::new();

    //This will hold the highest experience
    let mut highest_experience = 0;

    //This will store the information of the developer with the current highest years of experience
    let mut developer_with_the_highest_experience = vec!["".to_string()];

    //Number of Developers to be interviewed
    let num_to_be_interviewed = ask_for_input("How many Developers are going to be interviewed ?");
    let mut num_to_be_interviewed:i32 = num_to_be_interviewed.parse().expect("Invalid Number");

    while num_to_be_interviewed > 0 {
        //The current developer information
        let mut developer_info: Vec<String> = Vec::new();

        let name = ask_for_input("Developer Name ?");
        developer_info.push(name);

        let age = ask_for_input("Developer Age ?");
        developer_info.push(age);

        let gender = ask_for_input("Is the Developer a Male or Female");
        developer_info.push(gender);

        let marital_status = ask_for_input("Is He/She married or single");
        developer_info.push(marital_status);

        let programming_lang = ask_for_input("What Programming Languages does the Developer use?");
        developer_info.push(programming_lang);

        let experience = ask_for_input("How many years of experience does the Developer have ?");
        developer_info.push(experience);

        let phone_num = ask_for_input("Developer's Phone Number");
        developer_info.push(phone_num);

        let email = ask_for_input("Developer's Email");
        developer_info.push(email);

        let github_acct = ask_for_input("Developer's Github Account Name");
        developer_info.push(github_acct);

        //Stores the current developer's information
        all_developers_info.push(developer_info.clone());

        //Update the developer's with the highest years of experience
        let developer_experience: i32 = developer_info.clone()[5].parse().expect("Invalid number");
        if developer_experience > highest_experience {
            highest_experience = developer_experience;
            developer_with_the_highest_experience = developer_info ;
        }

        num_to_be_interviewed -= 1;
    }

    //Print's the Developer with the highest years of experience information
    println!("The developer with the highest years of experience is:
    Name: {}
    Age: {}
    Gender: {}
    Marital Status: {}
    Programming Language: {}
    Experience: {}
    Phone Number: {}
    Email: {}
    Github Account: {}"
    , developer_with_the_highest_experience[0]
    , developer_with_the_highest_experience[1]
    , developer_with_the_highest_experience[2]
    , developer_with_the_highest_experience[3]
    , developer_with_the_highest_experience[4]
    , developer_with_the_highest_experience[5]
    , developer_with_the_highest_experience[6]
    , developer_with_the_highest_experience[7]
    , developer_with_the_highest_experience[8]);
    
}

fn ask_for_input(que: &str) -> String {
    println!("{}", que);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim().to_lowercase();
    input
}
