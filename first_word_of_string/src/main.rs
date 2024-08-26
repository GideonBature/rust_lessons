fn main() {
    let s = String::from("Gideon Bature");
    let word = first_word(&s);
    println!("s: {}", s);
    println!("word: {}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
