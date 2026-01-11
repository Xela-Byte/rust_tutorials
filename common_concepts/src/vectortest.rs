pub fn test_vec_int() {
    let mut vector_result: Vec<i32> = Vec::new();

    println!("{:?}", vector_result);
    println!(
        "Capacity: {}, Length: {}",
        vector_result.capacity(),
        vector_result.len()
    );
}

#[derive(Debug)]
struct Car {
    manufacturer: String,
    model: String,
}

pub fn test_vec_string() {
    let mut car_list: Vec<Car> = vec![];
    let mut car_list2: Vec<Car> = vec![];

    for _ in 1..=1_000_000u32 {
        car_list.push(Car {
            manufacturer: String::from("BMW"),
            model: String::from("M4"),
        });
    }

    for _ in 1..=1_000_000u32 {
        car_list2.push(Car {
            manufacturer: String::from("Hyundai"),
            model: String::from("Sonata"),
        });
    }

    let keep_car = |e: &Car| {
        if e.manufacturer == "BMW" {
            return true;
        } else {
            return false;
        }
    };

    println!("Length: {:?}", car_list.len());
    println!("Capacity: {:?}", car_list.capacity());
}
