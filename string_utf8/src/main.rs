fn main() {
    // creating a string
    let s = String::new();
    println!("{}", s);

    // create string using to_string()
    let mut name = "Gideon Bature".to_string();
    println!("{}", name);

    // create string from
    let mut s1 = String::from("I love Rust");
    println!("{}", s1);

    // update a string
    s1.push_str(" and Aptos Move");
    println!("{}", s1);

    // update a single character
    name.push('!');
    println!("{}", name);

    // concatenation of string
    let first_name = String::from("Gideon");
    let last_name = String::from("Bature");
    let full_name = first_name + " " + &last_name;
    println!("{}", full_name);

    let one = String::from("1");
    let two = String::from("2");
    let three = String::from("3");

    // let combined: String = one + &two + &three;
    // println!("{}", combined);

    let combined1 = format!("{}-{}-{}", one, two, three);
    println!("{}", combined1);
    
    let slice = &combined1[0..2];
    println!("{}", slice);

    for c in s1.chars() {
        println!("{}", c);
    }

    for b in s1.bytes() {
        println!("{}", b);
    }

}
