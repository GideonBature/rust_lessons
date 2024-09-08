#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    name: String,
    age: u8
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.width > other.width && self.height > other.height {
            true
        } else if self.width == other.width && self.height == other.height {
            true
        } else {
            false
        }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

impl Person {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_name(&self) {
        println!("Name: {}", self.name);
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50
    };

    // println!("The area of the rectangle is {} square pixels.", area(&rectangle));

    // println!("rectangle: {:#?}", rectangle);

    let area = rectangle.area();
    println!("The area of the rectangle is {} square pixels.", area);

    let mut person = Person {
        name: String::from("Bene Elohim"),
        age: 30
    };

    println!("person: {:#?}", person);

    person.set_name(String::from("Gideon Bature"));

    println!("person: {:#?}", person);

    person.get_name();

    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    let can_hold = rectangle.can_hold(&rect1);
    println!("Can rect1 hold rectangle? {}", can_hold);

    let square = Rectangle::square(10);

    println!("square: {:#?}", square);

    let area_of_square = square.area();
    println!("The area of the square is {} square pixels.", area_of_square);

}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
