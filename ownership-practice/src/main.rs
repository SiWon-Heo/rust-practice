// 1. String type
let mut s = String::from("hello");
s.push_str(", world!");
println!("{}", s);

// 2. Move
let x = 5;
let y = x;

let s1 = String::from("hello");
// ptr -> [0]: h, [1]: e, ..., [4]: o
// len: 5
// capacity: 5
let s2 = s1;

// 3. Clone
let s1 = String::from("hello");
let s2 = s1.clone();

// 4. Ownership Function
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

// 5. Return values
fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String{
    let some_string = String::from("yours");
    
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// 6. Reference Borrowing
fn main () {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("the length of '{}' is {}.", s2, len);
}

fn calculate_length(s:String) -> (String, usize){
    let length = s.len();
    (s,length)
}

// 7. Mutable Reference
fn main() [
    let mut s = String::from("hello");
    change(&mut s);
]

fn change(some_string: &mut String){
    some_string.push_str(", world");
}

let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{},{}", r1, r2);
// >> ERROR

// 8. Dangling Reference
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
// >> ERROR ~~ This function's return type contains a borrowed value, but there is no value for it to be borrowed from
