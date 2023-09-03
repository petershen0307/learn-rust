use chapter10_traits::{NewsArticle, Summary, Tweet};

fn main() {
    // use_trait_in_parameter();
    traits_have_same_fn();
}

fn trait_example() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

fn default_implementation() {
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

fn default_implementation_interface() {
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

pub fn use_trait_in_parameter() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    notify(&tweet);
}

fn traits_have_same_fn() {
    let a = chapter10_traits::TraitsHaveSameFn {};
    // the chapter10_traits::TraitsHaveSameFn definition won't show the error, it can declare like this. Only ambiguous usage will show the error
    // when we only write `use chapter10_traits::Summary;`
    // rust compiler only compile chapter10_traits::Summary implementation
    // if we both write `use chapter10_traits::AnotherSummary;` at here, the rust compiler will show the ambiguous error.
    // use chapter10_traits::AnotherSummary;
    println!("{}", a.summarize_author());
}
