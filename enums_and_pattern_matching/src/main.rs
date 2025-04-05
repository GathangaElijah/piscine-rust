// **** Enums and Pattern Matching ****
// Enumerations are also called enums.
// They allow you to define a type by enumerating its possible variants.
// An enum value can either be one of its variants.

// **** Defining an Enum ****
enum IpAddrKind {
    V4(String),
    V6(String),
}
// You can put any data inside of an enum variant.

// **** The Option Enum and Its Advantages Over Null Values ****
// It encodes a scenerio where a value could be something or nothing.
enum Option<T> {
    Some(T),
    None,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    // **** Enum Values ****
    let four = IpAddrKind::V4; // They are namespaced under its identifier.
    let six = IpAddrKind::V6;

    // **** Structs vs Enums ****
    // We can put data directly into each enum variant.
    // Each variant can have different types and amounts of associated data.
    // We can define methods on enums just as we did on structs.

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(IpAddrKind::V4); // We can call a function with any of the variants.
    route(IpAddrKind::V6);

    // **** The Option Enum ****
    // The Option type encodes the very common scenario in which a value could be something or nothing.
    // Rust doesn’t have the null feature that many other languages have.
    // Null is a value that means there is no value there.
    // Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent.

    // **** The match Control Flow Operator ****
    // The match control flow operator allows you to compare a value
    // against a series of patterns and then execute code based on which pattern matches.
    // Patterns can be made up of literal values, variable names, wildcards, and many other things.

}

fn value_in_cents(coin: Coin) -> u8 {
    // The match expression is made up of arms.
    // An arm consists of a pattern and the code that should
    // be run if the value given to the beginning of the match
    // expression matches that arm’s pattern.
    // We can have as many arms as we need.
    // The code associated with each arm is an expression, and the
    // resulting value of the matched expression is the value of the
    // expression in the winning arm.
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn route(ip_kind: IpAddrKind) {}
