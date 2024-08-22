fn main() {
    println!("This is the main function");

    let name = String::from("Gideon Bature");
    let age = 32;

    another_function(name, age);

    let y = {
        let x = 3;
        x + 5
    };
    println!("The value of y is: {}", y);

    let sum = sum_param(5, 10);
    println!("The sum is: {}", sum);
}

fn another_function(name: String, age: i32) {
    println!("This is another function");
    println!("my name is: {}, and I am {} years old", name, age);

    let sum = sum();
    println!("The sum is: {}", sum);

    let (sum, difference) = calc(10, 5);
    println!("The sum is: {}", sum);
    println!("The difference is: {}", difference);
}

fn sum() -> i32 {
    5 + 7
}

fn sum_param(x: i32, y: i32) -> i32 {
    x + y
}

fn calc(x: i32, y: i32) -> (i32, i32) {
    let sum = x + y;
    let difference = x - y;
    (sum, difference)
    
}
