use std::time;

enum TrafficSignal {
    Red,
    Yellow,
    Green,
}

trait Duration {
    fn duration(self) -> time::Duration;
}

impl Duration for TrafficSignal {
    fn duration(self) -> time::Duration {
        match self {
            TrafficSignal::Red => time::Duration::from_secs(60),
            TrafficSignal::Yellow => time::Duration::from_secs(3),
            TrafficSignal::Green => time::Duration::from_secs(20),
        }
    }
}

fn main() {
    println!("Red duration {:?}", TrafficSignal::Red.duration());
    println!("Yellow duration {:?}", TrafficSignal::Yellow.duration());
    println!("Green duration {:?}", TrafficSignal::Green.duration());
}
