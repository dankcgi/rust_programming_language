// This annotation means an instance of ImportantExcerpt can’t outlive the 
// reference it holds in its part field.
struct ImportantExcerpt<'a> {
  part: &'a str,
}

pub fn result() {
  let novel = String::from("Call me Ishmael. Some years ago ...");
  let first_sentence = novel.split('.').next().unwrap();
  let i = ImportantExcerpt {
    part: first_sentence,
  };

  println!("First sentence: {}", i.part);
}