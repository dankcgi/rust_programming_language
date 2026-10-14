
pub fn initialise() {
  // initialise a new string, kind of like a vector, because it is a wrapper
  // for Vec<u8>
  let mut s = String::new();
  println!("{s}"); // will be a blank line
  
  // Loads data into the string from any type that implements the Display
  // trait, such as a string literal, with the .to_string method
  let data = "initial contents";
  s = data.to_string();
  println!("{s}");
  
  // Can also be called directly on the literal
  s = "initial contents".to_string();
  println!("{s}");
  
  // This is equivalent to using String::from
  s = String::from("initial contents");
  println!("{s}");
}

pub fn hello() {
  // String are UTF-8 encoded, so all the below are valid
  let hello = String::from("السلام عليكم");
  println!("{hello}");
  let hello = String::from("Dobrý den");
  println!("{hello}");
  let hello = String::from("Hello");
  println!("{hello}");
  let hello = String::from("שלום");
  println!("{hello}");
  let hello = String::from("नमस्ते");
  println!("{hello}");
  let hello = String::from("こんにちは");
  println!("{hello}");
  let hello = String::from("안녕하세요");
  println!("{hello}");
  let hello = String::from("你好");
  println!("{hello}");
  let hello = String::from("Olá");
  println!("{hello}");
  let hello = String::from("Здравствуйте");
  println!("{hello}");
  let hello = String::from("Hola");
  println!("{hello}");
}


