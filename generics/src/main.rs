struct Point<T> {
    x: T,
    y: T,
    z: T,
}

struct NewPoint<T, U, V> {
    x: T,
    y: U,
    z: V,
}

impl <T, U, V> NewPoint<T, U, V> {
    fn get_points(&self) -> (&T, &U, &V) {
        (&self.x, &self.y, &self.z)
    }

    fn mixer<W, X, Y>(&self, other1: NewPoint<W, X, Y>, other2: NewPoint<W, X, Y>) -> NewPoint<T, X, Y> where T: std::clone::Clone {
        NewPoint {
            x: self.x.clone(),
            y: other1.y,
            z: other2.z
        }
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p = Point { x: 5, y: 10, z: 20 };
    println!("p.x = {}, p.y = {}, p.z = {}", p.x, p.y, p.z);

    let p1 = Point { x: 4.3, y: 8.4, z: 15.2 };
    println!("p1.x = {}, p1.y = {}, p1.z = {}", p1.x, p1.y, p1.z);

    let p2 = Point { x: 'a', y: 'b', z: 'c' };
    println!("p2.x = {}, p2.y = {}, p2.z = {}", p2.x, p2.y, p2.z);

    let p3 = NewPoint { x: 5, y: 10.45, z: 'a' };
    println!("p3.x = {}, p3.y = {}, p3.z = {}", p3.x, p3.y, p3.z);

    println!("p3 points: {:#?}", p3.get_points());

    let p4 = NewPoint { x: 5, y: 10.45, z: 'a' };
    let p5 = NewPoint { x: 20, y: 7.45, z: 'b' };
    let p6 = NewPoint { x: 14, y: 15.22, z: 'c' };
    let p7 = p4.mixer(p5, p6);
    println!("p7.x = {}, p7.y = {}, p7.z = {}", p7.x, p7.y, p7.z);
}
