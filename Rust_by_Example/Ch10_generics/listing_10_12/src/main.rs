use aggregator::{Summary, Tweet, NewsArticle};


fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}",tweet.summarize());
    aggregator::notify(&tweet);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best \
hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());
    aggregator::notify(&article);
    
    // i32 does not implement Summary

    //let i = 5;
    //aggregator::notify(&i);
}

// function can't return different types

//fn returns_summarizable(switch: bool) -> impl Summary {
//    if switch {
//        NewsArticle {
//                headline: String::from("Penguins win the Stanley Cup Championship!"),
//                location: String::from("Pittsburgh, PA, USA"),
//                author: String::from("Iceburgh"),
//                content: String::from("The Pittsburgh Penguins once again are the best \
//hockey team in the NHL."),
//        }
//    } else {
//        Tweet {
//                username: String::from("horse_ebooks"),
//                content: String::from("of course, as you probably already know, people"),
//                reply: false,
//                retweet: false,
//            }
//    }
//}