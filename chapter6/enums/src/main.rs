enum IpAddrKind {
    V4,
    V6,
}

// functions can take an enum as a param.
fn route(_ip_kind: IpAddrKind) {}

// Rather than defining struct and including enum/kind as a type field,
// can instead define enum with data on it.
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
//
enum IpAddr {
    V4(String),
    V6(String),
}

// Variants can have different data attached to them.
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// enum can hold different structs, which may have different method implementations, etc.
struct Ipv4Addr {}
struct Ipv6Addr {}

enum IpAddrStdLib {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// It's possible to define a method on enum.
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

/* Alternative with 4 different structs results in each having own type and needing
   to code for each type.

    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

*/

fn main() {
    // create new instances of each variant
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    /* Struct approach

        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::v6,
            address: String::from("::1"),
        };

    */

    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));

    let _home = IpAddr2::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // using the Option enum.
    // enum Option<T> {
    //      None,
    //      Some(T),
    // }

    let some_number = Some(5);
    println!("sn: {:?}", some_number);
    // sn: Some(5)

    let some_char = Some('e');
    println!("sc: {:?}", some_char);
    // sc: Some('e')

    let absent_number: Option<i32> = None;
    println!("an: {:?}", absent_number);
    // an: None

    // Option<T> is better than having `null` because
    // Option<T> and T are different types and compiler can check it.

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // cannot add `Option<i8> to i8`
    // Rust doesn't understand how to add these two types together because they are
    // different types. Need to implement traits.

    if y.is_some() {
        println!("x plus y: {}", x + y.expect("should not be empty"));
    }

    if y.is_some() {
        println!("unwrap_or(0): x plus y: {}", x + y.unwrap_or(0));
    }

    let z: Option<i8> = None;

    if y.is_some() {
        println!(
            "unwrap_or_default(): x plus z: {}",
            x + z.unwrap_or_default()
        );
    }
}
