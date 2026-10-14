pub fn by_characters() {
  // To iterate Unicode scalar characters, use .chars 
  for c in "Зд".chars() {
    println!("{c}");
  }
}

pub fn by_bytes() {
  // To iterate over the raw bytes, use .bytes
  for b in "Зд".bytes() {
    println!("{b}");
  }
}