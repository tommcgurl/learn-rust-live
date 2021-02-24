use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("What up Twitch an Youtube!");

    let post = post.request_review();

    // First approval
    let post = post.approve();
    let post = post.approve();

    // Secon approval
    assert_eq!("What up Twitch an Youtube!", post.content());
}