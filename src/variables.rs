// Variables.

// Main function.
fn main() {

    // Creates an age variable sets the value to 21.
    let age = 21;

    // Print out the age.
    println!("Age: {}", age);

    // Can't re-assign a variable.
    // age = 22;

    // Use the mut keyword to be able to change the value.
    let mut name = "Jacob";

    // Print out the name.
    println!("Name: {}", name);

    // Now set name equal to James.
    name = "James";

    // Print out the name.
    println!("Now name equals: {}", name);

    // Use the const keyword to declare constants.
    // :f32 declares the type. This is a floating point value.
    const PI:f32 = 3.14;

    // Print out the constant.
    println!("Pi equals: {}", PI);

}