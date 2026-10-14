  pub fn appending() {
  // String can grow with .push_str
  let mut s = String::from("initial contents");
  s.push_str(" additional contents");
  println!("{s}");

  // push_str uses a slice to avoid ownership, meaning it's parameter can be
  // reused
  let mut s1 = String::from("foo");
  println!("s1 is {s1}");
  let s2 = "bar";
  s1.push_str(s2);
  println!("s2 is {s2}");
  println!("s1 is now {s1}");

  // A single character can be added to a string with .push
  let mut laugh_out = String::from("lo");
  laugh_out.push('l'); // note single quotes for char
  println!("{laugh_out}");
}

pub fn concatenating() {
  // Concatenate with +
  let hello = String::from("Hello, ");
  let world = String::from("world!");
  // hello wlll now be dropped, and world is a reference due to the signature
  // of the add method
  let hello_world = hello + &world;
  println!("{hello_world}");

  // Concatenate multiple strings with the format! macro
  let tic = String::from("tic");
  let tac = String::from("tac");
  let toe = String::from("toe");
  let tic_tac_toe = format!("{tic}-{tac}-{toe}");
  println!("{tic_tac_toe}");
}