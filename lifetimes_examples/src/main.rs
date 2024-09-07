use std::fmt::Display;

fn main() {
    let name = String::from("Gideon");
    let string = "Grace";

    let result = shortest_with_an_announcement (
        name.as_str(),
        string,
        "Today is someone's birthday",
    );
    println!("The shortest string is: {}", result);
}

fn shortest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() < y.len() {
        x
    } else {
        y
    }
}