// Owrnership is a set  of rules that governs how rust program manages memory.
// 

// **** The Stack and the Heap ****
// Stack and Heap are memories available to your code to use
// at run time. 
// A stack stores values in the order it gets them and removes the values in the opposite order.
// This is referred to as last in, first out.(LIFO)
// Adding data to a stack is called pushing onto the stack,
// and removing data is called popping off the stack.
// Data stored on the stack must have a known, fixed size.
// Data with unkwown size is stored at the heap.

// **** The Heap ****
// When you put data on the heap, you request a certain amount of space.
// The  memory allocator finds an empty spot in the heap that 
// is big enough, marks it as being in use and returns a pointer
// which is the address of that location.(Allocating in the Heap)
// Now because the pointer to the heap is a known fixed size,
// you can store the pointer on the stack, but when you want the actual data, 
// you must follow the pointer.

// Pushing to the stack is faster than allocating to the heap
// because the allocator never has to search for a place to store
// new data. The location is always at the top of the stack.
// In heap the allocator has to ffind a big enough space to hold
// data and to perform book keeping to prepare for the next allocation.

// Accessing data in the heap is slower because you have to follow
// a pointer to get there.

// When you call a function, every value passed to the function gets
// pushed into the stack. When the function is over, the values get popped off the stack.


// **** Rules of Ownership ****
// Rule 1: Each value in Rust has an owner.
// Rule 2: There can only be one owner at a time.
// Rule 3: When the owner goes out of scope, the value will be dropped.


fn main() {
    
    // **** Variable Scope ****
    let s = "hello"; // s is valid until the end of the scope.

    // **** String Type ****
    // :: is an operator that allows us to namespace this 
    //particular from function under the String type rather than using some sort of name like string_from.
    let mut s = String::from("hello"); // This string is mutable
    s.push_str(", world"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world`

    // **** Memory and Allocation ****
    // When a memory location is freed twice, thats an error and 
    // is called a double free error.
    let s1 = String::from("Hello Elijah!");
    let s2 = s1; // This is called a move. we have moved s1 to s2.
    println!("{s2}, world!"); // if we try to use s1 here, we get an error.

    // **** Scope and assignment ****
    // When a variable goes out of scope, Rust calls a special function for us.
    // The function is called drop. Drop clears memory and other resources
    // when a variable goes out of scope.
    // When a new value is assigned to and existing variable, 
    // the old value is dropped the memory is freed.

    let mut str = String::from("Hello");
    str = String::from("World"); // This is called a shadowing.
    println!("{str}, Elijah!");

    // **** Clone ****
    // If we want to copy the heap data of a string, including
    // the pointer, the length and the capacity, we can use a clone method. 

    let str_to_clone = String::from("Hello Elijah!");
    let str_cloned = str_to_clone.clone();
    println!("str_to_clone = {str_to_clone}, str_cloned = {str_cloned}");

    // **** Stack-Only Data: Copy ****
    // Types such as integers that have a known size at compile time
    // are stored entirely on the stack, so copies of the actual values are quick to make.
    // These types implement the Copy trait, and as a result, are not moved,
    // but rather copied.
    // Rust won't let us annotate a type with the Copy trait if the type, 
    // or any of its parts, has implemented the Drop trait.


    // **** Ownership and Functions ****
    // Passing a variable to a function will move or copy, just as assignment does.

    // **** Return Values and Scope ****
    // Returning values can also transfer ownership.
    // ownership of a variable the same pattern every time:
    // assigning a value to another variable moves it.
    // When a variable that includes data on the heap goes out of scope, 
    // the value will be cleaned up by drop unless ownership of the data has been moved to another variable.
}   
