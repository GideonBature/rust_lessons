use::std::fs::File;
// use::std::io::ErrorKind;

fn main() { 
    // let name_from_file = File::open("name.txt");

    // match name_from_file {
    //     Ok(file) => println!("File: {:?}", file),
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("name.txt") {
    //             Ok(fc) => println!("File created: {:?}", fc),
    //             Err(e) => println!("Error creating file: {:?}", e),
    //         }
    //         other_error => println!("Other error: {:?}", other_error),
    //     },
    // }

    // let file = File::open("items.txt").unwrap();
    // println!("File: {:?}", file);

    let file = File::open("items.txt").expect("Error opening file");
    println!("File: {:?}", file);


}
