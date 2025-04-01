
// Data Types


// Scalar Types
// A scalar type represents a single value.
// Rust has four primary scalar types:
// integers, floating-point numbers, Booleans, and characters.


// Integers
// Its  a number without a fractional component.
// Defualt is i32
// if an overflow occurs, the maximum value the type can hold, 
// the value will wrap to the minimum of the values that type can hold.


// Floating-Point Types
//All floating point numbers are signed.
// Its a number with a fractional component.
// Rust has two primitive types for floating-point numbers:
// f32 and f64, which are 32 bits and 64 bits in size, respectively.
// The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.

// Numeric Operations
// Rust supports basic mathematical operations.


// Booleans
// Booleans in rust has two possible values: true and false.
// them are one byte in size.


// Characters
// Its the most primitive alphabetic type.
// They are specified with single quotes, as opposed to string literals, which use double quotes.
// Has four bytes in size.
// Its represents a unicode scalar value.


// Compound Types


// Tuple Types
// A tuple is a general way of grouping together a number of values 
// with a variety of types into one compound type.
// They have a fixed length.
// We write a tuple as a comma-separated list of values inside parentheses.
// Each position in the tuple has a type, 
// and the types of the different values in the tuple don’t have to be the same.
// To get the individual values out of a tuple,
// we can use pattern matching to destructure a tuple value, 
// You can access the tuple element directly using a 
// period followed by the index of the value we want to access. a period.
// a tuple without values is called unit.
// the value and the corresponding type are both written as ().
// the empty tuple returns the unit value and does not return any other value.


// The Array Type
// Every element in an array must have the same type.
// Arrays have a fixed length.
//values in an array are stored in contiguous memory locations.
// values in an array can be accessed by indexing.
// they are written as a comma-separated list inside square brackets.
// You write arrays with square brackets, with the type of each
// element, a semicolon, and then the number of elements in the array.
// eg let a: [i32; 5] = [1, 2, 3, 4, 5];

fn main() {
    //Addition
    let sum = 5 + 10;

    // Subtraction
    let difference = 95.5 - 4.3;

    // Multiplication
    let product = 4 * 30;

    // Division
    let quotient = 56.7 / 32.2;
    let trucnated = -5 / 3;

    // Remainder
    let remainder = 43 % 5;


    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
}
