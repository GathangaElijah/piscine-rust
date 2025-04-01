
// If expressions
// blocks of code associated with the conditions in if expressions
// are sometimes called arms.
// else gives an alternative should the condition be false.
// if no else, the program will move on to the next line of code.
// the condition must be a bool.
// Multiple conditions are possible with else if.


// using if in a let statement
// if can be used on the right side of a let statement to assign the outcome to a variable.


// Repepting Code with loop


// Loop
// Executes the code in the loop body until the program explicitly stops the loop.

//While conditions
// While conditions are evaluated before the loop runs, 
// and the program enters the loop only if the condition evaluates to true.
fn main() {
    
    let number  = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
