
// Variables


// Variables are immutable by default
// You can make variables mutable by adding mut 
// infront of the variable name

// Constants


// This are valies that are bound to a name and are not 
// to change. They are always immutable.
// They are declared using he const keyword instead of let.
// Constants can be declared at any scope.
// Constants could be set only to a constant expression
// not the result of a value that could only be computed at runtime.
// eg const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// Constants are valid for the entire time a program runs,
// within the scope they were declared in.

// Shadowing

// You can declare a new variable with the same name as a previous variable.
// You can shadow a variable by using the same variable name
// and repeating the use of the let keyword.


fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
