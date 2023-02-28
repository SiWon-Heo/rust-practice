// 1. 선언
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main () {
    let user1 = User {
        email: String::from("adsf@adsf.com"),
        username: String::from("asdf"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("asdfadsf@asdf.com");
}

// 2. Field init shorthand
fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 3. Create Instances from other instances
fn main(){
    // --snip--
    let user2 = User{
        active: user1.active,
        username: user1.username,
        email: String::from("as@asdf.com"),
        sign_in_count: user1.sign_in_count,
    };

    // --snip--
    let user3 = User{
        email: String::from("vasdf@asdf.com"),
        ..user1
    };
}

// 4. Tuple Structs / Unit-Like Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

// Unit-like struct는 아직 데이터가 준비되지 않았을 때 선언만해둘 때 유용하게 사용 가능하다.
struct AlwaysEqual;
fn main() {
    let subject = AlwaysEqual;
}

// 5. Ownership of Struct
struct User {
    active: boot,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: "asdfasdf@asdf.com",
        username: "somename123",
        active: true,
        sign_in_count: 1,
    };
}
// >> ERROR : Missing Lifetime Specifier

// example
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the area of the rectangle is {} square pixels.", area(&rect1))
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// 6. Derived Traits
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: 32,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}

// 7. Methods
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    }

    println!("the area of the rectangle is {} square pixels", rect1.area);
}

// 8. More parameters
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main () {
    let rect1 = Rectangle {
        width: 30,
        height: 30,
    }
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    }
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    }

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}

