/*A Rust that ask the user for the number of sibling he/she has and
ask for information about each sibling and output the information.
Note: arr(i) is an array used to store information, where i is a integer , e.g arr1*/

use std::io;

fn main() {
    
    let num_siblings = ask_for_input("Welcome,How many siblings do you have");
    let num_sibling:i32 = num_siblings.parse().expect("Not a valid number");

    //This will run for each sibling the user has.
    for i in 1..num_sibling + 1 {

        println!("For sibling {}", i);
        let name_of_sibling = ask_for_input("What is the name of sibling");

        let age = ask_for_input("Age of sibling");
        let age:i32 = age.parse().expect("Invalid age");

        if age > 18 {

            let marital_status = ask_for_input(
       "\nSibling marital status:Input married or single"
            );


            if marital_status == "single" {

                let work_status = ask_for_input(
                    "\nSibling working status:Input worker or student"
                );

                if work_status == "student" {

                    let university = ask_for_input("What is your University");
                    let course = ask_for_input("What is your course of study");
                    let arr1 = [name_of_sibling, age.to_string(), marital_status, work_status, university, course];

                    println!("\nName: {}\nAge: {}\nMarital Status: {}\nWorking Status: a {}\nUniversity: {}\nCourse:{}\n",
                    arr1[0], arr1[1], arr1[2], arr1[3], arr1[4], arr1[5]);

                } else if work_status == "worker" {

                    let arr2 = [name_of_sibling, age.to_string(), marital_status, work_status];

                    println!("\nName: {}\nAge: {}\nMarital Status: {}\nWorking status: a {}\n",
                    arr2[0], arr2[1], arr2[2], arr2[3]);
                }


            } else if marital_status == "married" {

                let offsprings = ask_for_input(
                    "\nDoes your sibling have any offspring:Input yes or no."
                );

                let city = ask_for_input("What city does your sibling live in");
                let arr3 = [name_of_sibling, age.to_string(), marital_status, offsprings, city];

                println!("\nName: {}\nAge: {}\nMarital Status: {}\nHas any children: {}\nCity of residence: {}\n",
                arr3[0], arr3[1], arr3[2], arr3[3], arr3[4]);
                
            }


        } else {
            let weac_status = ask_for_input( 
                "\nDoes your sibling have a WAEC certification: Input yes or no");
               
                if weac_status == "yes" {

                    let secondary_school_attended = ask_for_input("What secondary school did your sibling attend");
                    let arr4 = [name_of_sibling, age.to_string(),weac_status, secondary_school_attended];
                    
                    println!("\nName:{}\nAge:{}\nWAEC Certified:{}\nName of Secondary School attended:{}\n",
                    arr4[0], arr4[1], arr4[2], arr4[3]);

                } else if weac_status == "no" || weac_status == "2" {

                    let current_class = ask_for_input("What is sibling current class");
                    let arr5 = [name_of_sibling, age.to_string(),weac_status, current_class];

                    println!("\nName:{}\nAge:{}\nWAEC Certified:{}\nCurrent Class:{}\n",
                    arr5[0], arr5[1], arr5[2], arr5[3]);

                    
                }
        }
    }
    
    

}

//A function that ask for input by passing a question as static string and return the input.
fn ask_for_input(que:&str) -> String { 

    println!("{}", que);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed read input");
    let input = input.trim().to_lowercase();
    input
}
