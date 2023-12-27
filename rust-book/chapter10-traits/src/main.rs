mod sections;
use sections::summary::Summary;

fn main() {}

#[test]
fn trait_example() {
    use sections::tweet::Tweet;
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

#[test]
fn default_implementation() {
    use sections::news_article::NewsArticle;
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

#[test]
fn default_implementation_interface() {
    use sections::tweet::Tweet;
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

#[test]
fn use_trait_in_parameter() {
    use sections::tweet::Tweet;
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    notify(&tweet);
}

#[test]
fn traits_have_same_fn() {
    use sections::test_trait_impl::TraitsHaveSameFn;
    let a = TraitsHaveSameFn {};
    // the sections::test_trait_impl::TraitsHaveSameFn definition won't show the error, it can declare like this. Only ambiguous usage will show the error
    // when we only write `use sections::summary::Summary;`
    // rust compiler only compile chapter10_traits::Summary implementation
    // if we both write `use sections::summary::AnotherSummary;` at here, the rust compiler will show the ambiguous error.
    // use sections::summary::AnotherSummary;

    // the better way is to use fully qualified syntax
    // https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
    println!("Summary::summarize_author={}", Summary::summarize_author(&a));
    println!("AnotherSummary::summarize_author={}", sections::summary::AnotherSummary::summarize_author(&a));
}

#[test]
fn overwrite_default_impl() {
    use crate::sections::test_trait_impl::OverWriteDefaultTraitImpl;
    let a = OverWriteDefaultTraitImpl {};
    println!("call summarize: {}", a.summarize())
}
