// **** Slice Type ****
// Slices let you reference a contiguous sequence of elements
//  in a collection rather than the whole collection.
// A slice is a kind of reference, so it does not have ownership.


fn main() {
    // **** String Slices ****
    // This is a reference to a part of a String.
    // Written as [starting_index..ending_index]
    // The slice data structure stores a reference to the first element and a length.
    // the length corresponds lastindex + 1 - startingindex
    // To specify the slice to start at 0, drop the starting value before the 2 dots.
    // To specify the slice to end at the end of the string, drop the ending value after the 2 dots.
    // you can drop both values to take a slice of the entire string.

    let s = String::from("Hello Elijah!");
    let hello = &s[0..5]; // This is a string slice.
    let world = &s[6..13]; // This is a string slice.

    let word = first_word(&s); // This is a string slice.
    println!("The first word is: {word}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // This converts the string to an array of bytes.
    for (i, &item) in bytes.iter().enumerate() { // This iterates over the array of bytes.
        if item == b' ' { // This checks if the current byte is a space.
            return &s[0..i]; // This returns a string slice.
        }
    }
    &s[..] // This returns a string slice.
}