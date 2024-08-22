fn main() {

    let (mut c, mut c3, mut c5, mut c35) = (0, 0, 0, 0);

    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
            c35 += 1;
        } else if i % 3 == 0 {
            println!("Fizz");
            c3 += 1;
        } else if i % 5 == 0 {
            println!("Buzz");
            c5 += 1;
        } else {
            println!("{}", i);
            c += 1;
        }
    }

    println!("c: {}, c3: {}, c5: {}, c35: {}", c, c3, c5, c35);
}
