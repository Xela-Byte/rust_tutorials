use std::cell::Cell;

pub struct Person<'p> {
    pub first_name: Cell<&'p str>,
    pub last_name: String,
    pub birth_year: u16,
    pub birth_month: u8,
}

pub fn new_person() -> Person<'static> {
    let person1 = Person {
        first_name: Cell::from("Xela"),
        last_name: String::from("Oladipupo"),
        birth_year: 2004,
        birth_month: 1,
    };

    person1.first_name.set("Jones");

    return person1;
}

#[derive(Debug)]
pub enum VehicleColor {
    Silver,
    Red,
    White,
    Green,
    Black,
    Titanium,
}

#[derive(Debug)]
pub struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    color: VehicleColor,
}

impl Vehicle {
    fn paint(&mut self) {
        self.color = VehicleColor::Silver
    }
}

fn new_vehicle() -> Vehicle {
    let mut vehicle_result = Vehicle {
        manufacturer: String::from("BMW"),
        model: String::from("M4"),
        year: 2000,
        color: VehicleColor::Black,
    };

    vehicle_result.paint();

    return vehicle_result;
}

pub fn create_vehicle() {
    let new_vehicle_result = new_vehicle();
    println!("{:?}", new_vehicle_result)
}
