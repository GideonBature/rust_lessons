// #[derive(Debug)]
// enum Info {
//     Student(String),
//     Teacher(u8),
//     School(String),
// }

// impl Info {
//     fn call(&self) {
//         println!("Info: {:?}", self);
//     }
// }

// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    // let student = Info::Student(String::from("Gideon Bature"));
    // let teacher = Info::Teacher(25);
    // let school = Info::School(String::from("Kaduna City"));

    // student.call();
    // teacher.call();
    // school.call();

    let some_number = Some(5);
    let some_string = Some("Hello, World!");
    let absent_number: Option<i32> = None;
    let four = 4;

    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);

    let sum = some_number.unwrap() + four;
    
    print!("sum: {}\n", sum);
}
