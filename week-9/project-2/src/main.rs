//A Rust program that ask for student's information and write them into a .xlsx file.
use std::io;
//rust_xlsxwriter is an external crate that is used for writing into a .xlsx file.
use rust_xlsxwriter::{Format, FormatAlign, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    //This create a new worksheet 
    let mut workbook = Workbook::new();
    //This adds a new worksheet to the worksheet
    let worksheet = workbook.add_worksheet();

    //This sets the columns width to 18
    worksheet.set_column_width(0, 18)?;
    worksheet.set_column_width(1, 18)?;
    worksheet.set_column_width(2, 18)?;
    worksheet.set_column_width(3, 18)?;
    //This is a center alignment format.
    let format = Format::new().set_align(FormatAlign::Center);

    //This merge columnn 0 - 3,names it "PAU SMIS" and format it to be center align.
    worksheet.merge_range(0, 0, 0, 3, "PAU SMIS", &format)?;
    //This writes the headings stored in heading array to row 1 in the worksheet.
    let heading = ["Student's Name", "Matric Number", "Department", "Level"];
    worksheet.write_row(1, 0, heading)?;

    //This will hold all the students information to recorded.
    let mut all_students_info: Vec<Vec<String>> = Vec::new();
    //Number of student to be recorded.
    let num_students_to_recorded:i32 = input("How many number of students are to be recorded")
        .parse()
        .expect("Invalid number");

    //This will loop for the number of students to be recorded.
    for _i in  0..num_students_to_recorded {

        //This will hold the current student information
        let mut student_info:Vec<String> = Vec::new();
        //These will ask for required information of the current student.
        let name = input("Student Name");
        let matric_no = input("Student's Matric number");
        let department = input("Student's Department");
        let level = input("Student's level");
        println!("\n");

        //These will store the information into the student_info vector.
        student_info.push(name);
        student_info.push(matric_no);
        student_info.push(department);
        student_info.push(level);

        //This stores the current information to all_students_info.
        all_students_info.push(student_info);


        let mut row = 2;
        //This write the current student's information to the worksheet starting from row 2.
        for student in &all_students_info {
            worksheet.write_row(row, 0, student)?;

            row += 1;
        }
    }

    //This saves the workbook as "SMIS.xlsx".
    workbook.save("SMIS.xlsx")?;
    //This return Result enum to the main function
    Ok(())
}

/*This gets input from the user by printing a question pass as a parameter
get a the reponse and returns it*/
fn input(que: &str) -> String {
    println!("{}", que);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim().to_lowercase();
    input
}
