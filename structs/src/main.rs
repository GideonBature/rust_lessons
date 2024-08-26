// struct of student with name, email, age and is_active
#[derive(Debug)]
struct Student {
    name: String,
    email: String,
    age: u8,
    is_active: bool
}

#[derive(Debug)]
struct RGB(u8, u8, u8);

#[derive(Debug)]
struct Person;

fn main() {
    let mut user1 = Student {
        name: String::from("Gideon Bature"),
        email: String::from("gideonbature@me.com"),
        age: 32,
        is_active: true
    };
    println!("user1: {:#?}", user1);

    user1.name = String::from("Grace Andrew");

    println!("user1: {:#?}", user1);

    let rgb1 = RGB(255, 0, 0);
    println!("rgb1: {:#?}", rgb1);

    let person = Person;

    println!("person: {:#?}", person);
}
