#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let a_name = Option::Some(String::from("Gideon Bature"));
    let no_name: Option<String> = Option::None;

    println!("a_name: {:#?}", a_name);
    println!("no_name: {:#?}", no_name);

    let success: Result<String, String> = Result::Ok(String::from("Success"));
    let error: Result<String, String> = Result::Err(String::from("Error"));
    println!("success: {:#?}", success);
    println!("error: {:#?}", error);
}
