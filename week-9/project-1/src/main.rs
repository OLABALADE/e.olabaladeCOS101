//A Rust Program that writes the beers of Nigerian Brewries sorted by type into a file.
use std::fs::File;
use std::io::Write;


fn main() {
    //This will hold the types of beers vectors.
    let mut beer_list:Vec<Vec<&str>> = Vec::new();

    //This will hold the types of Lager beers.
    let lager = vec!["Lager", "33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    beer_list.push(lager);

    //This will hold the types of Stout beers.
    let stout = vec!["Stout", "Legend", "Turbo", "Williams"];
    beer_list.push(stout);

    //This will hold the types of Non-Alcholoic drinks.
    let non_alcholic = vec!["Non-Alcoholic", "Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];
    beer_list.push(non_alcholic);

    //This create a text file named "NB_beers" and stores it a variable named file.
    let mut file = File::create("NB_beers.txt").expect("create failed");

    //The first loop will loop through each of the beer types vector
    for i in 0..beer_list.len() {
        //This stores the beer type name, which is the first item in each array.
        let beer_type = String::from(beer_list[i][0]) + "\n";

        //These will write into the file, the name of the beer type and add a separator to separate it from items.
        file.write_all(beer_type.as_bytes()).expect("Failed to write into file\n");
        file.write_all("*************\n".as_bytes()).expect("Failed to write into file\n");

        //The second loop will loop through the each item in each beer type vector
        for j in 1..beer_list[i].len() {
            //This will store each item of beer type vector from the first loop
            let beers = String::from(beer_list[i][j]) + "\n";
            
            //This will write each item into the NB_beer file.
            file.write_all(beers.as_bytes()).expect("Failed to write into file");

            /*This check if the item is the last item of the vector and it writes a line into the file
            so to separate next beer types list from the last one*/
            if j == beer_list[i].len() - 1 {
                file.write_all("\n".as_bytes()).expect("Failed to write into file\n");
            }
        }
    }
}
