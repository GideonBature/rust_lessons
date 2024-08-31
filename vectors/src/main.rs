fn main() {
    // empty vector
    let mut vec1: Vec<i32> = Vec::new();

    println!("{:?}", vec1);

    // vector with values
    let vec2: Vec<i32> = vec![1, 2, 3];

    println!("{:?}", vec2);

    vec1.push(10);
    vec1.push(20);
    vec1.push(30);

    println!("{:?}", vec1);

    vec1.pop();
    println!("{:?}", vec1);

    let second_elem = vec1[1];
    println!("{:?}", second_elem);

    let elem = vec1.get(5);
    
    match elem {
        Some(value) => println!("The value at index 5 is: {}", value),
        None => println!("There is no value at index 5"),
    }

    println!("Vector length: {}", vec1.len());

    let vec4 = vec![1, 2, 3, 4, 5];

    for elem in &vec4 {
        println!("{}", elem);
    }

    let mut vec5 = vec![10, 20, 30, 40, 50];

    for elem in &mut vec5 {
        *elem += 5;
    }

    println!("{:?}", vec5);

    #[derive(Debug)]
    #[allow(dead_code)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Int(7),
    ];

    println!("{:?}", row);


}
