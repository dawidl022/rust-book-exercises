const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    assert_eq!("irst-fay", to_pig_latin("first"));
    assert_eq!("apple-hay", to_pig_latin("apple"));
}

fn to_pig_latin(word: &str) -> String {
    let first_letter = match extract_first_letter(word) {
        Some(letter) => letter,
        None => return String::new(),
    };

    if VOWELS.contains(&first_letter) {
        format!("{}-hay", word)
    } else {
        let root_word = extract_non_first_letters(word);
        format!("{}-{}ay", root_word, first_letter)
    }
}

fn extract_first_letter(word: &str) -> Option<char> {
    word.chars().next()
}

fn extract_non_first_letters(word: &str) -> String {
    let mut result = String::new();

    for letter in word.chars().skip(1) {
        result.push(letter);
    }
    result
}
