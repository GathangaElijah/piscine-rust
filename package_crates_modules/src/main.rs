// **** Crates and Modules ****
// Packages: A Cargo feature that lets you build, test and share crates.
// Crates: A tree of modules that produce a library or executable.
// Modules and use: Let you control the organization, scope, and privacy of paths.
// Paths: A way of naming an item such as a struct, function or module.


// **** Packages and Crates ****
// A crate is the smallest amount of code that a rust compiler
// considers at a time.
// Crates can contain modules, and the modules may be defined
// in other files that get compiled with the crate.

// Binary Crates - These are programs that you can compile
// to an executable that you can run. eg CLI, Server.
// They must have a main function as an entry point.

//Library Crates - These are code designed for reuse in various projects.
// They don't have a main function.

// Crate root is a source file that the Rust compiler starts from and makes a crate from.

// A package is a bundle of one or more crates that provides a set
// of functionality. 
// It contains a Cargo.toml file that describes how to build the crates.
// A package can contain as many binary crates as you want and atmost one library crate.
// However, it must contain at least one crate, either library or binary.
// Using command 'cargo new my_project' creates a package that contains a library crate by default.
// A package can have multiple binary crates by placing files in the src/bin directory.
// Each file in src/bin is a separate binary crate.


// **** Modules ****
// Modules let you control the organization, scope and privacy of paths.
// use keyword is used to bring a path into scope.
// pub keyword is used to make items public.

// **** Modules cheat sheet ****
// 1. Start from the crate root.
// 2. Declare modules in the crate root file using the mod keyword.
// The compiler will look for the module's code in these places:
// - Inline, within curly brackets that replace the semicolon following mod.
// - In the file with the same name as the module, within src.
// - In the file with the same name as the module, within src/modules.
// 3. Declare submodules in their parent modules using the mod keyword.
// 4. Paths to code in modules are specified using the :: operator.
// Bring paths into scope with the use keyword.
// Once a module os part of your crate, you can refer to code
// in that module from any other part of that same crate,
// as long as the privacy rules allow, using the path to the code.

// Public vs Private: 
// A code is private by default.
// To make a module public, use the pub keyword before mod.eg pub mod name_of_module;.
// To make items within a public module public, use the pub keyword before the item's name.

// The use keyword: use keyword is used to bring a path into scope.

// **** Modules ****
// Modules allows you to control the organization,
// scope and privacy of paths.

// **** Paths ****
// A path can take two forms:
// - An absolute path is the full path starting from a crate root.
// for code from an external crate, the absolute path begins with the crate name.
// for code from the current crate, it starts with literal crate.

// - A relative path starts from the current module and uses self, 
// super or an identifier in the current module.

// Both relative and absolute are followed by one or more
// identifiers separated by double colons(::).
myrestaurant::front_of_house::hosting::add_to_waitlist();

fn main() {
    println!("Hello, world!");
}
