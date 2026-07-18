#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new (origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }

    fn change_destination (&mut self, new_destination: String) {
        self.destination = new_destination;
    } 

    fn increase_price (&mut self) {
        self.price *= 1.2;
    }

    fn itinerary (&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}

fn main() {

    let mut new_inst = Flight::new(String::from("Bengaluru"), String::from("Frankfurt"), 700.0, 1);
    println!("{:?}", new_inst);
    new_inst.change_destination(String::from("Munich"));
    println!("{:?}", new_inst);
    new_inst.increase_price();
    new_inst.itinerary();
    println!("{:?}", new_inst);

    let new_inst_2 = Flight {
        origin: String::from("Frankfurt"),
        destination: String::from("Switzerland"),
        ..new_inst
    };

    println!("{:#?}", new_inst_2);
}
