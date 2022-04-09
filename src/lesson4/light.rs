pub enum TrafficLight{
    Red,
    Green,
}

pub trait Time{
    fn get_time(color : TrafficLight) -> u8;
}

impl Time for TrafficLight{
    fn get_time(color : TrafficLight) -> u8 {
        match color {
            TrafficLight::Red =>  60,
            TrafficLight::Green =>  30,
        }
    }
}

fn main() {
    println!("Red light,duration is {}", TrafficLight::get_time(TrafficLight::Red));
    println!("Green light,duration is {}", TrafficLight::get_time(TrafficLight::Green));
}
