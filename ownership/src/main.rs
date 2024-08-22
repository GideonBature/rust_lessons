fn main() {
    let s = "Hello";
    println!("s: {}", s);

    let mut s = String::from("Hello");
    println!("s: {}", s);
    s.push_str(", World!");
    println!("s: {}", s);

    let s1 = String::from("Hello");
    let s2 = s1;
    println!("s2: {}", s2);

    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1: {}", s1);
    println!("s2: {}", s2);
}
