// 1. Enums with Data
enum IpAddrKind{
    V4,
    V6,
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
}

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
}

enum IpAddr{
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));

// 2. Enums with Data - different types
enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

enum IpAddr{
    V4(u8,u8,u8,u8),
    V6(String),
}

let home = IpAddr::V4(127,0,0,1);

let loopback = IpAddr::V6(String::from("::1"));

// 3. Option Enum
enum Option<T> {
    None,
    Some(T),
}

let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;

let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x+y;