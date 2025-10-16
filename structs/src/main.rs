struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn cal_area_rect() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:#?}");

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn main() {
    let user1 = build_user(
        String::from("xelaoaldipupo@gmail.com"),
        String::from("XelaByte"),
    );

    println!(
        "The user built is: {} {} {} {}",
        user1.email, user1.active, user1.sign_in_count, user1.username
    );

    cal_area_rect();
}
