fn main() {
    let mut counter = 0;

    loop {
        println!("counter: {}", counter);
        counter += 1;

        if counter > 20 { break };
    }

    let mut number = 3;

    while number != 0 {
        println!("number: {}", number);
        number -= 1;

        // wait for 1 second
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    let arr = [1, 2, 3, 4, 5];

    for elem in arr.iter() {
        println!("elem: {}", elem);
    }

    let name = "Gideon Bature";

    for c in name.chars() {
        println!("c: {}", c);
    }
}
