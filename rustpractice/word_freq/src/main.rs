fn most_frequent_word(text: &str) -> (&str, usize) {
    // Split the text into words and collect them into a vector
    let words: Vec<&str> = text.split_whitespace().collect();

    // Vector to store unique words and their respective counts
    let mut word_count: Vec<(&str, usize)> = Vec::new();

    // Iterate through each word in the text
    for &word in words.iter() {
        // Find if the word is already in the word_count vector
        let mut found = false;

        for &mut (existing_word, ref mut count) in word_count.iter_mut() {
            if existing_word == word {
                *count += 1;
                found = true;
                break;
            }
        }
// If the word wasn't found, add it to the vector with count 1
        if !found {
            word_count.push((word, 1));
        }
    }
// Find the word with the maximum frequency
    let mut max_word = "";
    let mut max_count = 0;

    for &(word, count) in word_count.iter() {
        if count > max_count {
            max_word = word;
            max_count = count;
        }
    }

    (max_word, max_count) // Return the word with the highest frequency and its count
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
