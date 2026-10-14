/*
 Convert strings to Pig Latin. The first consonant of each word is moved
 to the end of the word and ay is added, so first becomes irst-fay. Words
 that start with a vowel have hay added to the end instead (apple becomes
 apple-hay). Keep in mind the details about UTF-8 encoding!
*/
fn word_starts_with_vowel(word: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let first_letter = get_first_letter(word);

    vowels.contains(&first_letter)
}

fn get_first_letter(word: &str) -> char {
    word.to_lowercase().chars().nth(0).unwrap()
}

fn word_to_pig_latin(word: &str) -> String {
    let mut pig_latin = String::new();

    if word_starts_with_vowel(word) {
        let as_pig_latin = format!("{word}-hay");
        pig_latin.push_str(&as_pig_latin);
    } else {
        let mut word_string = word.to_string();
        let first = word_string.remove(0).to_string();
        let as_pig_latin = format!("{word_string}-{first}ay");
        pig_latin.push_str(&as_pig_latin);
    }

    pig_latin
}

fn sentence_to_pig_latin(text: &str) -> String {
    let mut pig_latin = String::new();

    for word in text.split_whitespace() {
        let pig_latin_word = word_to_pig_latin(word);
        pig_latin.push_str(&pig_latin_word);
        pig_latin.push_str(" ");
    }

    pig_latin.trim().to_string()
}

pub fn exercise_2() {
    println!("Exercise 2: convert a string to pig latin");
    let original = String::from("It was the best of times it was the worst of times");
    println!("Original: {original}");

    let result = sentence_to_pig_latin(&original);
    println!("Pig latin: {result}");
    println!("");
}
