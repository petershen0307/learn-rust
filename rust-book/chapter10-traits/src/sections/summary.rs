// Defining a Trait
pub trait Summary {
    fn summarize_author(&self) -> String;

    // Default Implementations
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub trait AnotherSummary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
