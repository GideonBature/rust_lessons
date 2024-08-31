// allow dead code and debug attributes
#![allow(dead_code)]
#[derive(Debug)]
enum TrafficLight {
    Red(TrafficLightAction),
    Yellow,
    Green,
}

#[derive(Debug)]
enum TrafficLightAction {
    Ready,
    Stop,
    Go,
}

fn main() {
    let light = TrafficLight::Red(TrafficLightAction::Stop);
    let light2 = TrafficLight::Green;

    if let TrafficLight::Red(action) = light2 {
        println!("Red light: {:?}", action);
    } else {
        println!("Not red light");
    }
}