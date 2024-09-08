mod restaurant {
    pub mod front_of_house {

        pub mod hosting {

            pub fn add_to_waitlist() {
                println!("Added to waitlist");
            }

            pub fn seat_at_table() {
                println!("Seated at table");
            }
        }

        pub mod serving {
                
                pub fn take_order() {
                    println!("Took order");
                }
    
                pub fn serve_order() {
                    println!("Served order");
                }
    
                pub fn take_payment() {
                    println!("Took payment");
                }
        }
    }

    mod back_of_house {
        #[allow(dead_code)]
        fn prepare_food() {}

        #[allow(dead_code)]
        fn wash_dishes() {}
    }
}

fn main() {
    restaurant::front_of_house::hosting::add_to_waitlist();
    restaurant::front_of_house::hosting::seat_at_table();
    restaurant::front_of_house::serving::take_order();
    restaurant::front_of_house::serving::serve_order();
    restaurant::front_of_house::serving::take_payment();
}
