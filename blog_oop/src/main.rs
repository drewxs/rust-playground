use blog_oop::Post;

fn main() {
    let mut post = Post::new();
    let s = "I ate a salad for lunch today";

    post.add_text(s);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    post.reject();
    post.request_review();
    post.approve();
    post.approve();
    assert_eq!(s, post.content());
}
