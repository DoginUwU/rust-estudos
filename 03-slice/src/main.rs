fn main() {
    let mut s1 = String::from("Hello World");
    let length = first_word_length(&s1);

    let s2 = s1.clone();
        
    s1.clear(); // s1 now is equal to "", but de length still the value
        
    println!("{}", length);
    println!("{}", &s2[6..11]); // Splice the string

    let word = first_word(&s2);
    
    println!("{}", word);
}

fn first_word_length(s: &String) -> usize {
    let bytes = s.as_bytes(); // Convert string to array of bytes
    
    for(i, &item) in bytes.iter().enumerate() { // Create iterator over array
        // .iter() -> return each element in a collection
        // .enumerate() -> wrap the result of iter and return as part of a tuple instead
        // first element is a index, and second is a reference of item
        if item == b' ' { // search for byte that represents space
            return i;
        }   
    }
    
    s.len() // return the length
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // Convert string to array of bytes
    
    for(i, &item) in bytes.iter().enumerate() { // Create iterator over array
        // .iter() -> return each element in a collection
        // .enumerate() -> wrap the result of iter and return as part of a tuple instead
        // first element is a index, and second is a reference of item
        if item == b' ' { // search for byte that represents space
            return &s[0..i];
        }   
    }
    
    &s[..]
}
