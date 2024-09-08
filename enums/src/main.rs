#[derive(Debug)]
#[allow(dead_code)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
#[allow(dead_code)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Traffic {
    light: TrafficLight,
    action: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("IPv4: {:?}\nIPv6: {:?}", four, six);

    let traffic_stop = TrafficLight::Red;
    action(traffic_stop);

    let traffic_go = TrafficLight::Green;
    action(traffic_go);

    let traffic_ready = TrafficLight::Yellow;
    action(traffic_ready);

    // delay for a second
    std::thread::sleep(std::time::Duration::from_secs(1));

    let traffic1 = Traffic {
        light: TrafficLight::Red,
        action: String::from("Stop"),
    };
    println!("\n{}", traffic1.action);

    // delay for a second
    std::thread::sleep(std::time::Duration::from_secs(1));

    let traffic2 = Traffic {
        light: TrafficLight::Yellow,
        action: String::from("Get ready to move"),
    };
    println!("\n{}", traffic2.action);

    // delay for a second
    std::thread::sleep(std::time::Duration::from_secs(1));

    let traffic3 = Traffic {
        light: TrafficLight::Green,
        action: String::from("Go"),
    };
    println!("\n{}", traffic3.action);

    // delay for a second
    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn action(traffic_type: TrafficLight) {
    println!("Traffic light is {:?}", traffic_type);
}
