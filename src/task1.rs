// Define the function concatenate_strings
fn concatenate_strings(string1: &str, string2: &str) -> String {
    // Create a new String to store the concatenated result
    let mut result = String::new();

    // Append the contents of string1 to result
    result.push_str(string1);

    // Append the contents of string2 to result
    result.push_str(string2);

    // Return the concatenated result
    result
}

fn main() {
    // Initialize two String variables
    let string1 = String::from("Hello, ");
    let string2 = String::from("world!");

    // Call the concatenate_strings function with references to string1 and string2
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Print the concatenated result
    println!("{}", concatenated_string);
}
