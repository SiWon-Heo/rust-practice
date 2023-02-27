// 1. first example
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

// 2. string slice problem
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}

fn main(){
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear() // >>> ERROR!
    println!("the first word is: {}", word);
}