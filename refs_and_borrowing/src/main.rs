fn main() {

    let mut name = String::from("Gideon");

    add_second_name(&mut name);

    println!("Full name: {:?}", name);

    let mut s = String::from("hello");

    let s1 = &s;
    let s2 = &s;

    println!("s1: {}, s2: {}", s1, s2);

    let s3 = &mut s;

    println!("s3: {}", s3);

    let new_string = no_dangle();

    println!("new_string: {}", new_string);

}

fn add_second_name(n: &mut String) {
    n.push_str(" Bature")
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}