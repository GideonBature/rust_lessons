fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    println!("Ate at restaurant");
    
}
mod front_of_house {
    use crate::eat_at_restaurant;
    pub mod hosting {
        pub fn add_to_waitlist() {
            super::eat_at_restaurant();
            println!("Added to waitlist");
        }
    }

}

