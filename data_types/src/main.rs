struct Person {
    name: String,
    age: u8
}
#[allow(dead_code)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green
}

fn main() {
    let decimal = 25_456;
    let hex = 0xdab;
    let octal = 0o77;
    let binary = 0b1010_1010;
    let byte = b'g';

    println!("decimal: {}, \nhex: {}, \noctal: {}, \nbinary: {}, \nbyte: {}", decimal, hex, octal, binary, byte);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("x: {}, \ny: {}", x, y);

    let sum= x + y;
    let difference = x - y;
    let product = x * y;
    let quotient = x / y;
    let remainder = x % y;
    println!("sum: {}, \ndifference: {}, \nproduct: {}, \nquotient: {}, \nremainder: {}", sum, difference, product, quotient, remainder);

    let t = true;
    let f: bool = false;
    println!("t: {}, \nf: {}", t, f);

    if t {
        println!("t is true");
    } else {
        println!("t is false");
    }

    let tup: (i32, f64, u8, char) = (500, 6.4, 1, 'G');
    let (w, x, y, z) = tup;
    println!("w: {}, \nx: {}, \ny: {}, \nz: {}", w, x, y, z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    let g = tup.3;
    println!("five_hundred: {}, \nsix_point_four: {}, \none: {}, \ng: {}", five_hundred, six_point_four, one, g);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("months[0]: {}, \nmonths[1]: {}", months[0], months[1]);

    for month in months.iter() {
        println!("month: {}", month);
    }

    let person = Person {
        name: String::from("Gideon Bature"),
        age: 30
    };
    println!("person.name: {}, \nperson.age: {}", person.name, person.age);

    let light = TrafficLightColor::Red;

    match light {
        TrafficLightColor::Red => println!("Stop!"),
        TrafficLightColor::Yellow => println!("Get ready to move"),
        TrafficLightColor::Green => println!("Go!")
    }
}
