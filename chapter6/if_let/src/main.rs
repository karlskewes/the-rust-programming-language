enum Coin {
    Dime,
    Quarter(State),
}

#[derive(Debug)]
enum State {
    _Alaska,
    Alabama,
}

fn main() {
    // match with catch all default verbose
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // `if let` same as match with 1 arm.
    // lose exhaustive checking that `match` enforces though.
    // bind variable (config_max => max)
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}")
    }

    let mut count = 0;

    let coin = Coin::Quarter(State::Alabama);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    // alternatively to match can if let ... else.
    let coin = Coin::Dime;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    println!("count: {count}");
}
