fn variables() {
    let x = 5;
    let y: i32 = 10;
    let z: f32 = 3.4;
    let a = 2.3;

    let mut mutable_number = 5;
    println!("Before mutation, mutable is: {}", mutable_number);
    mutable_number = 6;
    println!("After mutation, mutable is: {}", mutable_number);

    const ONE_MINUTE_IN_SECONDS: i32 = 60 * 60;

    println!(
        "x={}, y={}, z={}, a={}, constant(one minute in seconds)={}",
        x, y, z, a, ONE_MINUTE_IN_SECONDS
    );

    // Comments are ignored
    println!("A comment was ignored above!");
}

fn data_types() {
    let signed_number: i32 = 10;
    let unsigned_number: u32 = 300;
    let truthy_value: bool = true;
    let falsy_value: bool = false;
    let array_value: [i32; 5] = [1, 2, 3, 4, 5];
    let tuple_value: (i32, i32, i32) = (1, 2, 3);

    println!("Signed number: {}", signed_number);
    println!("Unsigned number: {}", unsigned_number);
    println!("Truthy value: {}", truthy_value);
    println!("Falsy value: {}", falsy_value);
    println!(
        "Array values: {}, {}, {}, {}, {}",
        array_value[0], array_value[1], array_value[2], array_value[3], array_value[4]
    );
    println!(
        "Tuple values: {}, {}, {}",
        tuple_value.0, tuple_value.1, tuple_value.2
    );
}

fn utility_functions(x: i32) {
    println!("The value passed from arg is: {}", x);
}

fn return_function(y: i32) -> i32 {
    y
}

fn print_function() {
    let z = return_function(56);
    println!("The returned value is: {}", z)
}

fn control_flow() {
    let num = 3;

    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let num1 = if condition { 5 } else { 4 };
    println!("The value of the number after condition is: {}", num1)
}

fn loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        };
    };

    println!("The result after the loop is: {}", result);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut num2 = 3;

    while num2 != 0 {
        println!("{num2}!");

        num2 -= 1;
    }

    println!("JAPA!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("FLY AWAY!!!");
}

fn convert_temp_to_fah(temp: f64) {
    let fah_temp: f64 = temp * (9.0 / 5.0) + 32.0;
    println!("The value of {temp}℃ converted from {fah_temp:.2}℉");
}

fn convert_temp_to_cel(temp: f64) {
    let cel_temp: f64 = (temp - 32.0) * (5.0 / 9.0);
    println!("The value of {temp}℃ converted is {cel_temp:.2}℉",);
}

fn main() {
    // Variables
    variables();
    // Data types (scalar and compound)
    data_types();
    // Functions
    utility_functions(100);
    // Print and Comments
    print_function();
    // If, expressions and loops
    control_flow();
    loops();
    // temp converter
    convert_temp_to_fah(12.0);
    convert_temp_to_cel(24.0);
}
