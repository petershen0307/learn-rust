pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub trait AnotherSummary {
    fn summarize_author(&self) -> String;
}

pub struct TraitsHaveSameFn {}

impl Summary for TraitsHaveSameFn {
    fn summarize_author(&self) -> String {
        String::from("TraitsHaveSameFn::Summary")
    }
}

impl AnotherSummary for TraitsHaveSameFn {
    fn summarize_author(&self) -> String {
        String::from("TraitsHaveSameFn::AnotherSummary")
    }
}
