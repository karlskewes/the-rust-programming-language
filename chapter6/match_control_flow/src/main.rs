#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // snip
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarters have design per state 1999->2008, encode value in enum.
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            // use {} if expression has multiple lines
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // `match` is exhaustive. If leave one 'arm' off then won't compile.
        // None => None, // pattern `None` not covered.
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("Value of penny: {}", value_in_cents(Coin::Penny));
    /*
    Lucky penny!
    Value of penny: 1
    */
    println!(
        "Value of Alabama Quater: {}",
        // value_in_cents(Coin::Quarter(UsState::Alabama))
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );
    /*
    State quarter from Alabama!
    Value of Alabama Quater: 25
    */

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => println!("add_fancy_hat()"),
        7 => println!("remove_fancy_hat()"),
        other => println!("move_player({other})"), // catch-all must go last, warning
                                                   // otherwise and other arms
                                                   // won't run.
    }
    // move_player(9)

    match dice_roll {
        3 => println!("add_fancy_hat()"),
        7 => println!("remove_fancy_hat()"),
        _ => println!("reroll()"), // catch-all without using value `_`
    }
    // reroll()

    match dice_roll {
        3 => println!("add_fancy_hat()"),
        7 => println!("remove_fancy_hat()"),
        _ => (), // do nothing, by specifying "the empty tuple type '()'"
    }
    // <no output>
}
