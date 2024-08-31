use std::collections::HashMap;

fn main() {
    let mut score = HashMap::new();

    score.insert(String::from("Gideon"), 100);
    score.insert(String::from("Bature"), 200);
    score.insert(String::from("Grace"), 300);
    score.insert(String::from("Elizabeth"), 400);
    score.insert(String::from("Will"), 500);

    println!("{:#?}", score);

    let key = String::from("Gideon");
    let my_score = score.get(&key).copied().unwrap_or(0);
    println!("Gideon's Scores is: {:#?}", my_score);

    let mut map = HashMap::new();

    map.insert(String::from("Blue"), 10);
    println!("{:#?}", map);

    map.insert(String::from("Red"), 40);
    println!("{:#?}", map);

    map.insert(String::from("Yellow"), 20);
    println!("{:#?}", map);

    map.insert(String::from("Green"), 30);
    println!("{:#?}", map);

    map.insert(String::from("Red"), 50);
    println!("{:#?}", map);

    map.entry(String::from("Red")).or_insert(60);
    println!("{:#?}", map);

    map.entry(String::from("Purple")).or_insert(70);
    println!("{:#?}", map);

    let mut map1 = HashMap::new();

    let text = "I love the rust programming language, and i am very optimistic that Rust will be one of the best if not the best very very soon, and there is no one that can stop that from happening, thank you so much rust, I love you Rust.";

    for word in text.split_whitespace() {
        let count = map1.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map1);
}
