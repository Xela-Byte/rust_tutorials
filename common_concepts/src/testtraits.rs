struct Person<PetType, CarType>
where
    PetType: Animal + Dangerous,
    CarType: Car,
{
    first_name: String,
    pet: PetType,
    car: CarType,
}

trait Car {
    fn print_car_name(&self) -> ();
}

trait Animal {
    fn make_sound(&self) -> ();
}

struct BMW {}

impl Car for BMW {
    fn print_car_name(&self) -> () {
        println!("BMW, zoosh!")
    }
}

trait Dangerous {}
trait NotDangerous {}

struct Cat {}
impl Animal for Cat {
    fn make_sound(&self) -> () {
        println!("Cat meowed!")
    }
}

struct Dog {}
impl Animal for Dog {
    fn make_sound(&self) -> () {
        println!("Dog barked!")
    }
}
impl Dangerous for Dog {}

pub fn create_person() {
    let pet_cat = Cat {};
    let pet_dog = Dog {};
    let person_car = BMW {};
    let person_result = Person {
        first_name: String::from("Xela"),
        pet: pet_dog,
        car: person_car,
    };

    person_result.pet.make_sound();
    person_result.car.print_car_name();
}
