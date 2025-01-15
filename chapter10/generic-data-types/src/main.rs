/* Standard library Option enum now makes sense, generic value type T.
enum Option<T> {
    Some(T),
    None,
}
*/

/* Standard library Result enum also makes sense, generic value type T and error E.
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

fn main() {
    single_type_functions();
    generic_functions();
    generic_structs();
    generic_struct_diff_fields();
    monomorphization();
}

/// Function Definitions for single type
fn single_type_functions() {
    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}

/// Generic Function definition
fn generic_functions() {
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        // consider restricting type parameter `T`
        let mut largest = &list[0];

        for item in list {
            //  binary operation `>` cannot be applied to type `&T`
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");
}

/// Generic Struct definitions
fn generic_structs() {
    struct Point<T> {
        x: T,
        y: T,
    }

    // methods on generic struct definition also require type annotation.
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // methods on generic struct definition can be implemented only for specific types
    // as well. Don't have to implement a method for all types.
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let _integer = Point { x: 5, y: 10 }; // Point<i32>
    let _float = Point { x: 1.0, y: 4.0 }; // Point<f64>

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

/// Generic Structs with differing field and method signature types
fn generic_struct_diff_fields() {
    // Method signature type parameters can differ from struct definition.
    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y, // type Y2 not type Y1
            }
        }
    }

    struct Point<T, U> {
        x: T,
        y: U,
    }
    // let wont_work = Point { x: 5, y: 4.0 }; // mismatched types...

    let _both_integer = Point { x: 5, y: 10 }; // MixedPoint<i32, i32>
    let _both_float = Point { x: 1.0, y: 4.0 }; // MixedPoint<f64, f64>
    let _integer_and_float = Point { x: 5, y: 4.0 }; // MixedPoint<i32, f64>

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // p3.x = 5, p3.y = c
}

/// Monomorphization: at compile time, generate typed functions and methods
/// required based on the usage of generics.
/// Therefore no runtime cost for generics.
fn monomorphization() {
    let integer = Some(5);
    let float = Some(5.0);

    // results in the following code (but likely different names):

    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }

    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
