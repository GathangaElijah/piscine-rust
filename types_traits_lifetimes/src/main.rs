// **** Generic types, traits and lifetimes ****
// Generics are abstract stand-ins for concrete types or other properties.

// **** Removing Duplications by extracting a function ****
// they allow us to replace specific types with  placeholder hat represents multiple
// types to remove duplication.

// **** Generic Data Types ****
// We use generics to creae definiions for items like function signatures or structs
// which we can the use with many different concrete data types.

// **** In Function Definitions ****
// we place genenrics in the signature of the function where we would usually specify
// the datatype of a parameter and the return value.


// **** Traits ****

// A trait defines the functionality a particular type has and can share
// with other types.
// Traits can be used to define shared behaviour in an abstract way.
// We can use trait bounds to show that a generic type can be 
// any type that has a certain behaviour.


// **** Defining a Trait ****
// Trait definitions are a way to group method signatures together to define
// a set of behaviours neccessary to accomplish some purpose.

// **** Traits as parameters ****
pub fn notify(item: &impl Summary){
    println!("Breaking new! {}", item.summarize());
}
// We specify the concrete type for the item parameter, we specify the
// the impl keyword and the trait name.
// This parameter accepts any type that implements the specified
// trait
// In the body of notify, we can call any methods on item that come from
// the Summary trait such as summurize.
// We can call notify and pass in any instance of NewsArticle or Tweet.

// **** Trait Bound Syntax ****
pub fn notify<T>:Summary>(item: &T) {
    println!("Breaking new! {}", item.summarize());
}

// **** Specifying Multiple trait Bounds with the + Syntax ****
// pub fn notify(item: &(impl Summary + Display)) {
// The + syntax is also valid with trait bounds on generic types:
// pub fn notify<T: Summary + Display>(item: &T) {


// **** Validating Refrences with Lifetimes ****
// Lifetimes ensure that references are valid as long as we
// need them to be.
// We must annotate types only when multiple types are possible.
// we must annotate lifetimes when the lifetimes of references could
// be related in a few different ways.
// Rust requires us to annotate the relationships using generinc
// lifetime parameters to ensure the actual references used at runtimr will definetly
// be valid.

// **** Preventing Dangling References with Lifetimes ****
// Dangling references cause a program to reference data other than the 
// data its intended to reference. 

// **** Life Annotation Syntax ****
// Life annotations dont change how long any of the references live,
// Rather, thet descrive the relationships of the lifetimes of multiple
// references to each other without affecting the lifetimes.
// The name of lifetime parameters must start with an apostrophe(')
// and they are usually all lowercase and very short, like the generic types
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// **** Lifetime Annotations in Function Signatures ****
// To declare lifetime annotations in function signatures,
// We need to declare lifetime parameters inside angle brackets
// between the function name and the parameter list.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// This means the returned reference will be valid
// as long as both the parameters are valid. 
// When annotating lifetimes in functions, the annotations go 
// in the function and not the function body
// When returning a reference from a function, the lifetime parameter
// for the return type needs to match the lifetime parameter
// for one of the parameters.
// If the reference returned does not refer to one of the parameters,
// it must refer to a value created within this function.

// **** Lifetime annotations in Struct Definitions ****
struct ImportantExcerpt<'a> {
    part: &'a str,
}
//Lifetimes on function or method parameters are called input lifetimes,
// and lifetimes on return values are called output lifetimes.

pub trait Summary {
    fn summarize(&self) -> String{
        String::from("(Read more...)")
    }
}
    // We have declared a trait using the trait keyword
    // and its name which is Summary.
    // we also create the trait as pub so that it can be 
    // by other crates depending on this crate.
    // We have declared the method signatures that describe
    // the behaviours of the types that implement this trait.
    // Instead of providing an implementation after the function signature
    // we use a semicolon.
    // Each type that implements the trait, must provide its own
    // custom behaviour for the body of the method.
    // The compiler enforces that any type that has the summary trait
    // will have the method summarize defined with this signature exactly
    // A trait can have multiple methods in its body
    // They are listed one per line and each line ends with a semicolon

// **** Implementing a trait on a type ****

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String{
        format!("{}, by {} ({})",
    self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String{
        format!( "{}: {}", self.username, self.content)
    }
}
// implementinf a trait on a type is similar to implementing regular methods.
// The only difference is that after impl, we put a trait name we want to implement,
// then we use the keyword for and specify the name of the type we want to 
// implement the trait for.
// We put the the method signatures as defined 
// Users of the crate can call the trait methods on instances of NewsArticle
// and Tweet in the same way we call regular methods.
// The difference os that the use must bring the trait into the scope as well as types. 

// **** Default implementations ****


use super::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    // Note we can implement a trait on a type only of either the trait
    // or the type or bith are local to our crate.
}
