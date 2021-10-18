#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

pub fn run() {
    let user1 = build_user(String::from("a@a.com"), String::from("keki"));

    let user2 = User {
        email: String::from("b@b.com"),
        username: String::from("bbb"),
        ..user1
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of rect1 is {}", rect1.area());
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
