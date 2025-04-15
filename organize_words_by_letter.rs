use std::collections::{HashMap};

fn main() {
    let words = vec![
        "apple".into(),
        "art".into(),
        "banana".into(),
        "Berry".into(),
        "Ninja".into(),
        "Karate".into(),
        "disco".into(),
        "Aardvark".into(),
        "dog".into(),
        "cherry".into(),
        "Cat".into()
    ];
    let output = organize_words(words);
    println!("{:?}", output);
}

fn organize_words(words: Vec<String>) -> HashMap<char, Vec<String>> 
{
    let mut map = HashMap::new();
    
    for word in words {
        if let Some(first_char) = word.chars().next() {
            let key = first_char.to_ascii_lowercase();
            map.entry(key).or_insert(Vec::new()).push(word);
        }
    }
    map
}
