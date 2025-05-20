fn main() {
    iterators_are_lazy();
}

fn iterators_are_lazy() {
    let v1 = vec![1, 2, 3]; // vec! returns a Vec<T>, .iter() accepts either array or Vec<T>,
    // hence useless clippy warning.

    // below doesn't do anything useful yet.
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
}

#[test]
fn immutable_values_iteration_demonstration() {
    let v1 = [1, 2, 3];

    let mut v1_iter = v1.iter(); // NOTE: iterator is mutable as .next() mutates internal
    // state (position in slice/array/vec).

    assert_eq!(v1_iter.next(), Some(&1)); // NOTE: IMMUTABLE references
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    println!("v1 can still be used: {v1:?}"); // not consumed by .iter()
}

#[test]
fn immutable_values_owned_copy_iteration_demonstration() {
    let v1 = [1, 2, 3];

    let mut v1_iter = v1.into_iter(); // NOTE: iterator is mutable as .next() mutates internal
    // state (position in slice/array/vec).

    assert_eq!(v1_iter.next(), Some(1)); // NOTE: MUTABLE references
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);

    println!("v1 can still be used: {v1:?}");
    // consumed by .into_iter(), but Copy Trait implemented for i32

    // i32 implements Copy Trait so can still be used.
    println!("v1 can still be used: {v1:?}"); // consumed by .into_iter()
}

#[test]
fn immutable_values_owned_no_copy_iteration_demonstration() {
    #[derive(Debug, PartialEq, PartialOrd)] // No Copy Trait
    struct NoCopy {
        value: i32,
    }

    let v1 = [
        NoCopy { value: 1 },
        NoCopy { value: 2 },
        NoCopy { value: 3 },
    ];

    let mut v1_iter = v1.into_iter(); // NOTE: iterator is mutable as .next() mutates internal
    // state (position in slice/array/vec).

    assert_eq!(v1_iter.next(), Some(NoCopy { value: 1 })); // NOTE: MUTABLE references
    assert_eq!(v1_iter.next(), Some(NoCopy { value: 2 })); // NOTE: MUTABLE references
    assert_eq!(v1_iter.next(), Some(NoCopy { value: 3 })); // NOTE: MUTABLE references
    assert_eq!(v1_iter.next(), None);

    println!("v1_iter cannot still be used: {v1_iter:?}");

    // borrow of moved value: `v1`
    // println!("v1 cannot still be used: {v1:?}"); // consumed by .iter()
}

#[test]
fn mutable_values_owned_no_copy_iteration_demonstration() {
    #[derive(Debug, PartialEq, PartialOrd)]
    struct NoCopy {
        value: i32,
    }

    let v1 = [
        NoCopy { value: 1 },
        NoCopy { value: 2 },
        NoCopy { value: 3 },
    ];

    let owned = |mut vals: [NoCopy; 3]| -> [NoCopy; 3] {
        for item in &mut vals.iter_mut() {
            *item = NoCopy {
                value: item.value + 10,
            };
        }

        // can still use mutated values here as have ownership in closure
        println!("vals mutated and can still be used: {vals:?}");

        // can return mutated values back to caller
        vals
    };

    let new_vals = owned(v1);

    // borrow of moved value: `v1`
    // println!("v1 cannot still be used: {v1:?}"); // consumed by .iter_mut()

    // returned values usable as expected.
    println!("new_vals: {new_vals:?}");

    let mut new_vals_iter = new_vals.iter();
    assert_eq!(new_vals_iter.next(), Some(&NoCopy { value: 11 })); // NOTE: IMMUTABLE references
    assert_eq!(new_vals_iter.next(), Some(&NoCopy { value: 12 })); // NOTE: IMMUTABLE references
    assert_eq!(new_vals_iter.next(), Some(&NoCopy { value: 13 })); // NOTE: IMMUTABLE references
    assert_eq!(new_vals_iter.next(), None);
}

#[test]
// "Consuming Adaptors" - Iterators that consume the iterator.
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: u32 = v1_iter.sum();
    // println!("v1_iter: {v1_iter:?}"); // v1_iter consumed by sum(); // "consuming adaptor"
    assert_eq!(total, 6);
}

#[test]
// "Iterator Adaptors" - Iterators that produce other iterators and DON'T consume the iterator.
fn iterator_map() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2_iter = v1.iter().map(|x| x + 1); // lazily produce new iterator v2
    println!("v1 still available: {v1:?}");
    println!("v2_iter: {v2_iter:?}");

    let v3: Vec<_> = v2_iter.collect(); // consume v2 into new collection

    assert_eq!(v3, vec![2, 3, 4]);
}

// Closures that capture their environment
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }
}
