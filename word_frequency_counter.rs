use std::collections::HashMap;

fn main() {
    let input = "Just a small town girl, living in a lonely world. She took a midnight train going anywhere.
    Just a city boy, born and raised in south Detroit. He took a midnight train going anywhere.";
    let freq = word_frequency(input);
    for (word, count) in freq {
        println!("{}: {}", word, count);
    }
}

fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    let lowercase = text.to_lowercase();
    for word in lowercase.split_whitespace() {
        let unpunctuated_word: String = word.chars().filter(|c|c.is_alphanumeric()).collect();
        if !unpunctuated_word.is_empty() {
            *freq.entry(unpunctuated_word).or_insert(0) += 1;
        }
    }
freq

}
