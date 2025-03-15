use listing_17_12b::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    println!("Current State = {}", post.state_name());

    post.request_review();
    assert_eq!("", post.content());
    println!("Current State = {}", post.state_name());

    post.reject();
    assert_eq!("", post.content());
    println!("Current State = {}", post.state_name());

    post.request_review();
    assert_eq!("", post.content());
    println!("Current State = {}", post.state_name());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
        println!("Current State = {}", post.state_name());

}
