use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("What up Twitch an Youtube!");
    assert_eq!("", post.content());


    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("What up Twitch an Youtube!", post.content());
}