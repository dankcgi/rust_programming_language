pub trait Summary {
  fn summarise_author(&self) -> String;

  fn summarise(&self) -> String {
    format!("(Read more from {}...)", self.summarise_author())
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarise_author(&self) -> String {
    format!("{}", self.author)
  }
}

pub struct SocialPost {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub repost: bool,
}

impl Summary for SocialPost {
  fn summarise_author(&self) -> String {
    format!("@{}", self.username)
  }
}

pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarise());
}