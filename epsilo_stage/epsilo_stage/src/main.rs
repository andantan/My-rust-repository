fn main() {
    let s = String::from("Hello world!");

    let word = first_word(&s[..]);

    println!("word: {}", word);
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}