fn main() {
    let num = 57;

    if num % 2 == 0 {
        println!("{} is an even number", num);
    } else {
        println!("{} is an odd number", num);
    }

    let is_even = if num % 2 == 0 { true } else { false };
    println!("{} is an even number: {}", num, is_even);

    let a = 50;
    let b = 10;
    let c = 20;

    if a < b && b < c {
        println!("{} is less than {} and {} is less than {}", a, b, b, c);
    } else {
        println!("{} is not less than {} or {} is not less than {}", a, b, b, c);
    }

    let name = "Gideon";

    match name {
        "Gideon Bature" => println!("Hello, Gideon Bature!"),
        _ => println!("Hello, {}!", name),
    }
}
