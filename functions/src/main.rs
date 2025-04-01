
// Funtions
//The main function is the entry point of many programs.
// the fn keyword allows you to declare a new function.
// snake case is the most common style in Rust for function and variable names.
// We declare functions in rust by entering the fn
// followed by the function name and a set of parentheses.
// we call a function by entering its name followed by a set of parentheses.


// Parameters
// In function signatures, you must declare the type of each parameter.
// When providing many parameters, seperate the parameter declarations with commas.


// Statements and Expressions
// Statements are instructions that perform some action and do not return a value.
// eg creating a variable and assigning a value to it.
// Expressions evaluate to a resulting value. eg 5 + 6.
// expressions can be part of statements.
// expresions do not include ending semicolons.


//Functions with Return Values
// We don’t name return values, but we do declare their type after an arrow (->).
// Rust returns the value of the final expression in a function.
// You can return early from a function by using the return keyword 
// and specifying a value, but most functions return the last expression implicitly.

fn main() {
    another_function(5,'h');
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");
}

fn another_function(x: i32, unitlable: char) {
    println!("The measuremenr is: {x}{unitlable}");
}

fn five() -> i32 {
    5
}
