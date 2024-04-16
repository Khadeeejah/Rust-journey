// write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
// If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.

fn first_word(s: &String) -> &str {
    // convert our String to an array of bytes using the as_bytes method.
    let bytes = s.as_bytes();
    //  create an iterator over the array of bytes using the iter method:
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
