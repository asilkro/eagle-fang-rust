use std::collections::HashSet;

fn main() {
    let input = vec![
        "miyagi".to_string(),
        "cobra".to_string(),
        "eagle".to_string(),
        "miyagi".to_string(),
        "eagle".to_string(),
        "fang".to_string(),
    ];
    let output = remove_string_duplicates(input);
    println!("{:?}", output);
}

fn remove_string_duplicates(words: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for word in words {
        if seen.insert(word.clone())
        // HashSet has ownership of the value so cloning is probably the quickest way to do this.
        // Changing the order of operations and storing into result then referencing it would be an alternative.
          // Using a Vec instead of a HashSet could work with a .contains() but it would get unwieldy quickly on longer lists.
        {
            result.push(word);
        }
    }
  result
