// **** Structs ****
// Its a custom data type tha lets you package together
// and name multiple related values that make up a meaningful group.

// **** Defining and Instantiating Structs ****
// To define a struct, we enter the keyword struct followed by the  name of the struct.
// we name the fields of the struct inside the curly brackets.
// The fields are defined in key: value pairs. where name is the key and value is the type.
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
 // **** Tuple Structs without Named Fields ****
// Tuple structs have the added meaning the struct name provides 
// but don’t have names associated with their fields;
// Very iseful when you want to give the whole tuple a name
// and make the tuple a different type from other tuples.

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// **** Unit-Like Structs Without Any Fields ****
// They behave like (), the unit type.
// They are useful when you need to implement a trait on some type
// but don’t have any data that you want to store in the type itself.

struct AlwaysEqual;

// **** Ownership of Struct Data ****
// Using a String type rather than &str is because we want a struct to own all of its data.


// **** Calculate the Area of a rectangle ****
#[derive(Debug)] // Specifies how to format the struct when printing it.
struct Rectangle {
    width: u32,
    height: u32,
}

// **** Methods ****
// They are similar to functions
// The difference is that they are defined within the context of a struct.
// Their first parameters are always self, which represents
// the instance of the struct the method is being called on.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // Methods can take multiple parameters.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// Anything in the implementation block will be associated with 
// Rectangle type.
// Methods can take ownership of self,
// borrow self immutably, 
// or borrow self mutably.
// if you want to change the instance that you’re calling the method on
// as part of what the method does, you need to use &mut self as the first parameter.
// This technique is usually used when a method transforms self into something else,
// and you want to prevent the caller from using the original instance after the transformation.


// **** Associated Functions ****
// All functions defined within an impl block are called 
// associated functions because they’re associated with the type named after the impl.
// Associated functions don’t have self as their first parameter,
// are often used for constructors that will return a new instance 
// of the struct.
impl Rectangle {
    fn square(size: u32) -> self{
        self {
            width: size,
            height: size,
        }
    }
}
// to calll the associated function we use the :: syntax with the struct name.
// eg let sq = Rectangle::square(3);


fn main() {
    
    // We can create an instance of the struct by using the name of the struct
    let mut user1 = User {
        email: String::from("elijahgathang@gmail.com"),
        username: String::from("Elijah"),
        active: true,
        sign_in_count: 1,
    };
    // To get a specific value in a struct, we use dot notation.
    user1.email = String::from("elijahgathang@gmail.com"); // This is an error.
    // The entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
    
    // You can create an instance from another instance by using struct update syntax.
    let user2 = User {
        active: user1.active,
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        sign_in_count: user1.sign_in_count,
    };
    // the syntax .. specifies that the remaining fields not explicitly 
    //set should have the same value as the fields in the given instance.
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // user3 is a new instance of the User struct.
    // it has a different email but has the same values for the other fields as user1.

    // **** Using Tuple Structs without Named Fields to Create Different Types ****
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // black and origin are different types.
    // when destructuring struct tuples, you are required to name
    // the type of the struct 
    let Point(x, y, z) = origin;

    let subject = AlwaysEqual; // This is an instance of a struct without any fields.


    // **** Calculating the Area of a Rectangle with a Function ****
    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    println!(
        "The area of the rectangle is {} square pixels.", 
        rect1.area() // This is a method call.
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // Rust uses automatic derefrencing to make method calls more convenient.
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    // **** Meaningful display with derived traits ****
    println!("rect1 is {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user(email: String, username: String) -> User{
    // having to declare the struct fields by name is annyoing.
    // User {
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 1,
    // }

    // We can use the field init shorthand syntax.
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
