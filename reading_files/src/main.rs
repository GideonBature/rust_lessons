use std::fs::File;
// use std::io::{self, Read};
use std::io::{self, BufRead, BufReader};


// Version 1

/*
fn main() -> io::Result<()> {
    read_file();

    Ok(())
}

fn read_file() -> io::Result<()> {
    let mut f = File::open("resume.txt")?;

    let mut content = String::new();

    f.read_to_string(&mut content)?;

    println!("{}", content);

    Ok(())
}
*/

// Version 2

/*
fn main() {
    match read_file_to_string("resume.txt") {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Error: {}", e),
    }

}

fn read_file_to_string(filename: &str) -> io::Result<String> {
    let mut f = File::open(filename)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}
*/

// Version 3

fn main() -> io::Result<()> {
    let f = File::open("resume.txt")?;

    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}