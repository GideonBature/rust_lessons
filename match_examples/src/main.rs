// #[derive(Debug)]
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(Rarity),
// }

// #[derive(Debug)]
// enum Rarity {
//     Common,
//     Uncommon,
//     Rare,
//     Epic,
//     Legendary
// }

fn main() {
//    let coin = Coin::Quarter(Rarity::Epic);

//    let value = value_in_coin(coin);

//    println!("The value of the coin is: {}", value);


    let die_roll = 20;

    match die_roll {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        _ => println!("Other"),
    }
}

// fn value_in_coin(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(rarity) => {
//             println!("This quarter is {:?}", rarity);
//             25
//         },
//     }
// }
