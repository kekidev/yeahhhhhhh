enum IpAddrKind {
    V4,
    V6
}

enum Move {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

enum IpAddr2 {
    V4(String),
    V6(String)
}

enum IpAddr3 {
    V4(u8,u8,u8,u8),
    V6(String)
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

pub fn run() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let home3 = IpAddr3::V4(127,0,0,1);

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    let y: Option<i8> = Some(5);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let mycoin: Coin = Coin::Quarter(UsState::Alaska);

    let some_u8_value = 8; // idk
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("idk"), 
    }

    let some_u8_value2 = Some(0u8);
    if let Some(3) = some_u8_value2 {
        println!("three");
    }

    // this equals to below code
    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    // let mut count = 0;
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     count += 1;
    // }

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(num) => Some(num + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}












