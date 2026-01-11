struct Person {
    first_name: String,
    last_name: String,
}

pub fn test_closures() {
    let add = |x, y| x + y;

    let result = add(2, 2);
    let print_result = |x: i32| {
        println!(
            "This result of the add closure plus {0} is: {1}",
            x,
            (result + x)
        )
    };

    print_result(56);

    let mut person1: Person = Person {
        first_name: String::from("Xela"),
        last_name: String::from("Oladipupo"),
    };

    let mut change_name = |new_last_name: &str| person1.last_name = String::from(new_last_name);

    change_name("Xlebyte");
    println!("Person {}", person1.last_name)
}
