// **** Collections ****
// vectors : allows you to store a variable number of values next to each other.
// strings : collection of characters.
// hash maps : allows you to associate a value with a particular key.

fn main() {
    // **** Vectors ****
    // Vec<T> - This can hold any type.
    // It allows you to store more than one value in a 
    // single data structure that puts all the values next 
    // to each other in memory.
    // Vactors can only store values of the same type.
    // They are useful when you have a list of items in order.

    // **** Creating a New Vector ****
    // We usee Vec::new() to create a new vector.
    let v: Vec<i32> = Vec::new(); 
    // Vector s are implemented using generics
    // We can also use the vec! macro to create a new vector
    // to hold the values you give it.
    let v = vec![1, 2, 3];
    // Rust infers the type of the vector from the values you give it.

    // **** Updating a Vector ****
    // We can add elements to a vector using the push method.
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // **** Reading Elements of Vectors ****
    // We can get elements of a vector using the index operator
    // or using the get method.
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; 
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    // The two methods are provided so that you can choose 
    // how the program behaves when you try to use an index value 
    // that is outside the range of the existing elements.

    // **** Iterating over the Values in a Vector ****
    // We can iterate over the values in a vector using a for loop.
    let v = vec![100,32, 57];
    for i in &v {
        println!("{i}")
    }
    // We can also iterate over mutable references to the elements in a vector.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    // **** Using an Enum to Store Multiple Types ****
    // We can use an enum to store multiple types in a vector.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // **** Dropping a Vector Drops Its Elements ****
    // When a vector goes out of scope, all of its contents are also dropped, 
    // just like any other struct.
}
