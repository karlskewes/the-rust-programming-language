#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "params: The area of the rectangle is {} square pixels.",
        area_params(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "tuple: The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "rectangle: The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    // println!("rectangle is {}", rect1); // `Rectangle` doesn't implmement `std::fmt::Display`

    // println!("rectangle is {rect1:?}"); // `Rectangle` doesn't implement `Debug`

    // After adding outer attribute to struct it now works.
    println!("rectangle is {rect1:?}");
    // rectangle is Rectangle { width: 30, height: 50 }

    println!("rectangle is {:#?}", rect1);
    /*
    rectangle is Rectangle {
        width: 30,
        height: 50,
    }
    */

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // [src/main.rs:51:12] 30 * scale = 60
        height: 50,
    };

    dbg!(&rect2); // note dbg! prints to stderr not stdout
    /*
    [src/main.rs:55:5] &rect2 = Rectangle {
        width: 60,
        height: 50,
    }
    */
}

fn area_params(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
