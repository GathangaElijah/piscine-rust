// **** Smart Pointers ****
// A pointer is a varible that contains an address in memory.
// The address points to some other data.
// The common type of pointer in rust is the reference '&' and they borrow the value
// they point to.
// Smart pointers are data structures that not only act like a pointer
// but also have additional metadata and capabilities.
// A reference counting type pointer enables you to allow data to have multiple owners
// by keeping track of the number of owners and when no owners remain, 
// the data is cleaned up.

// One difference  between references and smart pointers is that references
// are pointers that only borrow data while smart pointers own the data they point to.
// Smart pointers are usually implemented using structs.
// Smart pointers implement the 'Deref' and 'Drop' traits.
// The 'Deref' trait allows an instance of the smart pointer
// to behave like a reference so you can write your code to work with either 
// references or smart pointers.
// The 'Drop' trait allows you to customize the code that is run when an instance
// of the smart pointer goes out of scope.

// **** Enabling Recursive types with boxes ****

// A recursive type can have another value of the same type as part of itself.

// **** Cons List ****

// A cons list is a data structure that comes from the Lisp programming language.
// It is a list that contains two elements: the value of the current item and the next item.
// e.g (1, (2, (3, Nil)))
// The last item in the list contains only a value called Nil without a next item.
// The cons list is a recursive type.
// in Rust Vec<T> is a better choice than cons list.
// Boxes provide only the indirection and heap allocation.
// They are useful in cases like the cons list where indirection is needed
// but ownership is not.
// The 'Box<T>' type is a smart pointer because it implements the 'Deref' rait 
// that allows boxes to be treated like references.
// and 'Drop' traits when the value goes out of scope, the heap data will be cleaned up.


// **** Treating Smart Pointers Like Regular References with the Deref Trait ****

// The 'Deref' trait allows you to customize the behavior of the dereference operator *.
// The dereference operator * is an operator that follows a pointer to the value the pointer is pointing to.
// The 'Deref' trait is used to override the * operator.

// **** Defininf our own smart pointer ****
    struct MyBox<T>(T);
    impl <T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    // We define a type MyBox<T> that will hold one value of type T.
    // The type is a tuple struct with one element.
    // The 'new' function takes one parameter of type T and returns a MyBox instance that holds it.
    // if we call MyBox::new in main, we get a compilation error because 
    // Rust doesn't know how to dereference a value of type MyBox.

// **** Treating a type like a reference by implementing the Deref trait ****
    use std::ops::Deref;
    impl <T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    // The type Target = T; type definition is required for implementing the Deref trait.
    // Without the deref method, the compiler can only dereference & references.
    // The deref method gives the compiler the ability to take a value of any type that implements Deref 
    // and call the deref method to get a & reference that it knows how to dereference.
    // The reason the deref method returns a reference to a value, and that the plain dereference outside
    // the parentheses in *(y.deref()) is still necessary is the ownership of the value.
    // If the deref method returned the value directly instead of a reference to the value, 
    // the value would be moved out of self. 
    // We don't want to take ownership of the inner value inside MyBox<T>.
    //

// **** Implicit Deref Coercions with Functions and Methods ****

// Deref coercion is a reference to a value that implements the Deref trait into a reference to another type.
// Deref coercion can convert a &String to a &str because String implements the Deref trait
// so that &String values can be treated as &str values.
// It happens automatically when we pass a reference to a particular type to
// a function or method that doesn't match the parameter type in the function or method definition.
// A sequence of calls to the deref method converts the type we provided into the type
// the parameter needs.

// **** How Deref Coercion Interacts with mutability ****

// Similar to how you use the Deref trait to override the * operator on immutable references, 
// you can use the DerefMut trait to override the * operator on mutable references.
// Rust does deref coercion when it finds types and trait implementations in three cases:
// - From &T to &U when T: Deref<Target=U>
// - From &mut T to &mut U when T: DerefMut<Target=U>
// - From &mut T to &U when T: Deref<Target=U>

// **** Running code on cleanup with the Drop trait ****

// Drop lets you customize what happens when a value is about to go out of scope.
// You can provide an implementation for the Drop trait on any type.
// It can be used  to release resources like files or network connections.
    struct CustomerSmartPointer {
        data: String,
    }
    impl Drop for CustomerSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomerSmartPointer with data `{}`!", self.data);
        }
    }
    // The Drop trait is included in the prelude.
    // The Drop trait requires you to implement one method named drop that takes a mutable reference to self.
    // Variables are dropped in the reverse order they are created.

// **** Dropping a value early with std::mem::drop ****
// It is used when you want to release resources early so that another code in the same
// scope can acquire the resources.
// you use std::mem::drop to force the value to be dropped before the end of the scope.
// The std::mem::drop function is different from the drop method in the Drop trait.
// W call it by passing as an argument the value we want to force drop

// **** Rc<T>, the Reference Counted Smart Pointer ****

// This allows you to have multiple owners of some data.
// You explicitly enable multiple ownership by using the Rc<T> type.
// The Rc<T> type keeps track of the number of references to a value
// to determine whether or not the value is still in use.
// If there are zero references to a value, the value can be cleaned up
// without any references becoming invalid.
// We use the Rc<T> when we want to allocate some data on the heap for
// multiple parts of our program to read and we can't determine at compile 
// time which part will finish using the data last.
// Note that Rc<T> is only for use in single-threaded scenarios.

//**** using Rc<T> to share data ****

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc;

// we add use statement to bring Rc<T> into scope because it is not in the prelude.
// in main, if we want other variables to share the data, we use clone.
// eg let b = Rc::clone(&a);
// This doesn't make a deep copy of all the data like the clone method does.
// It only increaments the reference count.


fn main() {

    // **** Using Box<T> to Point to Data on the Heap ****

    // Box<T> allows you to store data on the heap rather than the stack.
    // What remains on the stack is the pointer to the heap data.
    // Boxes don't have performance overhead.
    // They are used in the following situations:
    // - When you have a type whose size can't be known at compile time and
    // you want to use a value of that type in a context that requires an exact size.
    // - When you have a large amount of data and you want to transfer ownership
    // but ensure the data won't be copied when you do so.
    // - When you want to own a value and you care only that it's a type that implements
    // a particular trait rather than being of a specific type.

    // **** Using a Box<T> to store Data on the heap ****

    let b = Box::new(5); // This defines a variable b that points to a value of 5 on the heap.
    // When b goes out of scope, it will be deallocated.
    // The deallocation happens for the box (b)- stored on the stack and the 
    // data(5) - stored on the heap.

    
}
