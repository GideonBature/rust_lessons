pub mod network {
    fn connect() {
        println!("Connection established!");
    }

    pub fn initiate_connection() {
        println!("Initiating connection...");
        // wait for 2 seconds
        std::thread::sleep(std::time::Duration::from_secs(1));
        connect();
    }
}

fn main() {
    network::initiate_connection();
}
