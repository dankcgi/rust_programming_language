use std::collections::HashMap;

pub fn overwrite() {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25);

  println!("{scores:?}");
}

pub fn add_if_not_present() {
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);

  // No key for Yellow, so key and value added
  scores.entry(String::from("Yellow")).or_insert(50);
  // Key Blue already exists, so not changed
  scores.entry(String::from("Blue")).or_insert(50);

  println!("{scores:?}");
}

pub fn update_based_on_old_value() {
  let text = "hello world wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{map:?}");
}