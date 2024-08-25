fn main() {
    let arr: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let slice = &arr[1..=4];

    println!("slice: {:?}", slice);

    let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    let slice0 = &vec[0..=3];

    println!("slice0: {:?}", slice0);

    let s = String::from("hello world");
    let hello = &s[0..=4];
    let world = &s[6..=10];
    println!("hello: {}, world: {}", hello, world);

    let name = String::from("Gideon Bature");
    let first_name = &name[..6];
    print!("first_name: {}", first_name);

    let last_name = &name[7..];
    println!("\nlast_name: {}", last_name);

    // This codebase demonstrates the use of slices in Rust
    // Here we're creating a slice of the entire 'name' string
    let full_name = &name[..];
    println!("full_name: {}", full_name);

    // We're also showing how slices work with arrays
    let dogs = ["Buddy", "Max", "Charlie", "Rocky", "Bailey"];
    // This creates a slice of the entire 'dogs' array
    println!("Dogs: {:?}", &dogs[..]);

    // The main purpose of this code is to illustrate different ways
    // of creating and using slices in Rust, including slices of
    // strings, vectors, and arrays.

    
}
