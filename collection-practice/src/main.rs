// 1. Vector
let v: Vec<i32> = Vec::new();

let v = vec![1,2,3];

let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);

{
    let v = vec![1,2,3,4,5];
} // v goes out and drop

let v = vec![1,2,3,4,5];
let third: &i32 = &v2;
println!("the third element is {}", third);

match v.get(2) {
    Some(third) => println!("the third element is {}", third),
    None => println!("there is no third element"),
}


let mut v = vec![1,2,3,4,5];
let first = &v[0];
v.push(6);
println!("the first element is: {}", first);
// >> ERROR!


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];


// 2. String
let mut s = String::from("foo");
s.push_str("bar");

let mut s = String::from("lo");
s.push("l");

let s1 = String::from("hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;

let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);


