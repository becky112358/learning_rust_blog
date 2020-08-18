mod method_state_pattern;

fn main() {
    let mut post = method_state_pattern::Post::new();

    post.add_text("Go green!");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Go green!", post.content());
    println!("{}", post.content());
}
