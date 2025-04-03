fn main() {
    // **** References and Borrowing ****
    // A reference is like a pointer in that it’s an address we can follow
    //  to access the data stored at that address that the pointer points to.
    // The data is owned by some other variable.
    // Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type.

    let s1 = String::from("Hello Elijah!"); // This defines a string and binds it to variable s1
    let len = calculate_length(&s1); // This passes a reference to s1 to the function calculate_length
    // This action is called borrowing.
    println!("The length of '{s1}' is {len}");

    // you cannot modify something you borrowed
    let mut s = String::from("Hello");
    // change(&s);  // This will throw an error.
    change(&mut s); // This will work.

    // References are immutable by default.
    // If you have a mutable refrence to a value, you can have no
    // other refrences to that value

    // This two fails
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // We overcome this by creating another scope

    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    println!("{},",  r2); //But now r1 is out of scope.

    // We also cannot have a mutable references while we have
    // an immutable one to the same value.
    // A refrence scope starts from where it is introduced and
    // continues through the last time that reference is used.
    
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}