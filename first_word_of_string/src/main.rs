fn main() {
    let mut s = String::from("Gideon Bature");
    let word1 = first_word_1(&s);
    println!("s: {}", s);
    println!("word: {}", word1);

    let s1 = String::from("Hello, world!");
    let word2 = first_word_2(&s1);
    println!("s1: {}", s1);
    println!("word: {}", word2);

    s.clear();
    let word3 = first_word_1(&s);
    println!("s: {}", s);
    println!("word: {}", word3);
}

fn first_word_1(s: &str) -> usize {
    let bytes = s.as_bytes();
    println!("bytes: {:?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, item) in bytes.iter().enumerate() {
        if item == &b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}