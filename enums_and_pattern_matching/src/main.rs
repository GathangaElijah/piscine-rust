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

    // **** Patterns and Matching ****
    // The match arms must cover all possibilities.
    // Patterns are a special syntaxt for matching against the structure of types,
    // including both complex and simple types.
    // A pattern may consist of some combinations of the following:
    // - literals
    // - destructured arrays, enums, structs, or tuples
    // - variables
    // - wildcards
    // - placeholders
    // To use a pattern we compare it to some value.
    // if the pattern matches the value we use the value parts in our code.
    //

    // **** All the Places Patterns Can Be Used ****
    
    // - match arms
    // - conditional if let expressions
    // - while let conditional loops
    // - for loops
    // - let statements
    // - function parameters

    // 1. match arms
    // We use patterns in the arms of match expressions.
    // these are expressions defined with the 'match' keyword,
    // a value to match on, and one or more arms that consist of a pattern
    // and the code to run if the value matches that arm's pattern.
    // eg match value {
    //     pattern => expression,
    //     pattern => expression,
    // }
    match x {
        None => None, // None is the pattern and on the where the arrow points to is the expression.
        Some(i) => Some(i + 1), // Some(i) is the pattern and on the where the arrow points to is the expression.
        _ => None, // _ is the wildcard pattern.
        // The underscore, _, is a special pattern that matches any value and does not bind to that value.
        // This tells Rust we won’t use the value, so Rust won’t warn us about an unused variable.
        // The underscore pattern will match any value.)
    }
    // 2. conditional if let expressions
    // if let matches only one case, and the pattern can only match one value.
    // it can be combined with 'else if', 'else if let' arms, 
    // and an 'else' block to contain code to run if the pattern doesn’t match.

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if  is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
    // The difference between match and if let is that match checks all 
    // patterns and if let checks only one.

    // 3. while let conditional loops
    // while let is like a combination of 'while' and 'if let'.
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    })
    while let Ok(val) = rx.recv() {
        println!("Got: {val}");
    }

    // 4. for loops
    // the value that follows the keyword for is a pattern.
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    // 5. let statements
    // let PATTERN = EXPRESSION;

    // 6. function parameters
    // Function parameters can also be patterns.

    // **** Refutability: Whether a Pattern Might Not Match ****
    // Patterns come in two forms: refutable and irrefutable.
    // Patterns that will match for any possible value passed are irrefutable.
    // Patterns that can fail to match for some possible value are refutable.
    // Function parameters, let statements, and for loops can only accept irrefutable patterns.
    // if let and while let expressions can accept refutable and irrefutable patterns.


    // **** Pattern Syntax ****
    // 1. Matching Literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    // The syntax is useful when you want your code to take an action
    // it gets an particular concrete value.

    // 2. Matching Named Variables
    // These are irrefutable patterns that match any value.
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);
    // When you use named variables in match, if let or while, they are shadowed
    // because each of this constructs introduces a new scope.

    // 3. Multiple Patterns
    // You can match multiple patterns using the | syntax. The pattern or operator.
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    // This means that if x matches either one or two, the arm will run.

    // 4. Matching Ranges of Values with ..=
    // The ..= syntax allows you to match a range of values.
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // 5. Destructuring to Break Apart Values
    // We can use patterns to destructure structs, enums, and tuples to
    // use different parts of these values.
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    // This creats the variables a and b that match the values of the x and y
    // fields of the p struct.

    // 6. Destructuring Enums
    // Destructuring an enum depends on the way the data is stored within the enum definition.
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => println!("Text message: {text}"),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
    // The number of variables in the pattern must match the number of fields in the struct.

    // 7. Destructuring Nested Structs and Enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    // 8. Destructuring Structs and Tuples

    // 9. Ignoring Values in a Pattern
    // You can ignore values in a pattern by using the _ pattern.
    // It takes into account all the remaining possible values
    // you can use the _ pattern to ignore any value not specified.
    // You can also use the _ pattern to ignore unused variables.
    // you can also use the .. pattern to ignore multiple values.

    // 10. Extra Conditionals with Match Guards
    // A match guard is an additional if condition specified after the pattern in a match arm.
    // It must use a conditional expression that must evaluate to a bool.

    // 11. @ Bindings
    // The at operator (@) lets us create a variable that holds a value at the
    // same time we’re testing that value to see whether it matches a pattern.

}

fn route(ip_kind: IpAddrKind) {}
