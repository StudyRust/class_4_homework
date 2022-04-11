enum TrafficLight {
    Red,
    Green,
    Yellow
}

impl TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 10,
            TrafficLight::Yellow => 3,
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;
    println!("time of red light is {:?}", red.time());
    println!("time of green light is {:?}", green.time());
    println!("time of yellow light is {:?}", yellow.time());
}
