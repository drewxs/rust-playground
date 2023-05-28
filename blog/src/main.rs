use blog::Post;

fn main() {
    let mut post = Post::new();
    let s = "I ate a salad for lunch today";

    post.add_text(s);
    let post = post.request_review();
    let post = post.approve();

    assert_eq!(s, post.content());
}
