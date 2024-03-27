/*
Here’s a small programming problem: write a function that takes a
    string of words separated by spaces and returns the first
    word it finds in that string. If the function doesn’t find
    a space in the string, the whole string must be one word,
    so the entire string should be returned.
*/

fn main() {

    let s = String::from("Hello there!");
    println!("{}", first_word(&s));
}

/// returns the index of the end of the first word
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    // loop over bytes to find a space
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    // return whole word if not found
    s
}
