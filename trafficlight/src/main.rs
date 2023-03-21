#[derive(Debug)]
enum TrafficLight {
    Green,
    Red,
    Yellow,
}

trait Duration {
    fn return_duration(&self) -> u8;
}

impl Duration for TrafficLight {
    fn return_duration(&self) -> u8 {
        match self {
            &Self::Green=>7,
            &Self::Red=>120,
            &Self::Yellow=>20,
        }
    }
}
fn main() {
    println!("green is {:?}s",TrafficLight::Green.return_duration());
    println!("red is {:?}s",TrafficLight::Red.return_duration());
    println!("yellow is {:?}s",TrafficLight::Yellow.return_duration());
}
