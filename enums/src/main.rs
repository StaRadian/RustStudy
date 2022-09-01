#[derive(Debug)] // So we can inspect the state in a minute
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_state) => {
            //println!("State quarter from {:?}!", state);  //??
            25
        }
    }
}
fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    let dice_roll = 7;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn add_fancy_hat() {
    println!("add hat");
}
fn remove_fancy_hat() {
    println!("remove hat");
}
fn move_player(num_spaces: u8) {
    println!("hat: {num_spaces}");
}
