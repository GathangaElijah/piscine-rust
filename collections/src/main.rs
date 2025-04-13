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

    // **** Strings ****
    // Strings are implemented as a collection of bytes.
    // We have only one string type ie "str".
    // 'String' type is growable, mutable, owned, utf-8 encoded string type.
    //

    // **** Creating a new slice ****
    let mut s = String::new();
    // It creates a new, empty string called s.
    // If we have some initial data that we want to 
    // initialize the string with, we use 'to_string method'.
    let data = "intial contents";
    let s = data.to_string();
    // The method also works directly
    let s = "initial Contents".to_string();
    // It creates  a string containing inital contents.

    // we can also use 'String::from' to create a string from a literal.
    let s = String::from("initial contents");

    // **** Updating a string ****
    // You can use '+' or 'format!' to concatenate a string values.

    // **** Appending to a string with push_str and push ****
    // We can grow a string using the 'push_str' method to append a string slice.
    let mut s = String::from("foo");
    s.push_str("bar");
    // push_str does not take ownership of the parameter.

    // The 'push' method takes a single character as a parameter
    // and adds it to the string.
    let mut s = String::from("lo");
    s.push('l')
    // The result will contain lol.


    // **** Concatenation with the + operator or format! Macro ****
    // Using the '+'
    let s1 = String::from("Hello, "); 
    let s2 = String::from("World!"); 
    let s3 = s1 + &s2 //Remember s1 has been moved here and it can no longer be used
    // s3 contains "Hello World!"
    // We only ass a '&str' to a 'String' We can't add two 'String' values together.
    // IF we want to concatenate multiple strings, 
    // the behaviour of the '+' operator gets unwieldy
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // For readability, use 'format!'
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    // Instead of format! returning the output to the screen. 
    // It returns the string with the contents.
    // format! uses refrences so it doesnt take ownership of any of its paramenters


    // **** Indexing into Strings ****
    // You do not index strings with an index operator in rust.

    // **** Slicing Strings ****
    // Rather than using [] with a single number,
    // You can use [] with a range to create a string slice containing particular bytes
    let hello = "Some words in hindi"
    let s = &hello[0..4];
    // Here s will be a &str that contain the first four bytes of the string.

    // **** Iterating over the strings ****
    // use 'chars' for unicode scalar values. You can iterate over the elements to access each element.

    for c in "3$".chars(){
        println!("{c}");
    }
    // the 'bytes' method returns each raw byte

    // **** Hash Maps ****
    // The type 'HashMap<K, V>' store a mapping of
    // keys of type K to to values of type v using a hashing function
    // which determines how it places the keys and values into memory.
    // Useful when you want to check out data using keys and not indexes.
    // A key could be of any type.

    // **** Creating a new hashmap ****
    // using 'new' and add elements with 'insert'
    // Maps stores data on the heap
    // All of the keys must have the same type

    // **** Accessing values in the Hash Map ****
    // using the key to the get method
    // We can iterate using for loop

    // Types that implement the copy trait are copied in the hashmap.
    // Owned values will be moved and the hash map will be the owner.
    // EAch key stores a unique value
    // 'entry' method takes the key you want to check as a parameter.
    // The return is an enum called Entry that 
    // represents a value that might or might not exist.
}
