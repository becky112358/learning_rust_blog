mod method_state_pattern;
mod method_states_and_behaviours_as_types;

fn main() {
    method_state_pattern_immediate_approve();

    method_state_pattern_initial_rejection();

    method_states_and_behaviours_as_types();
}

fn method_state_pattern_immediate_approve() {
    let mut post = method_state_pattern::Post::new();

    post.add_text("Go green!");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Go green!", post.content());
    println!("{}", post.content());
}

fn method_state_pattern_initial_rejection() {
    let mut post = method_state_pattern::Post::new();

    post.add_text("How about blue?");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.add_text(" A little bit of blue, together with green.");
    assert_eq!("", post.content());

    post.request_review();

    post.approve();
    assert_eq!("How about blue? A little bit of blue, together with green.",
               post.content());
    println!("{}", post.content());
}

fn method_states_and_behaviours_as_types() {
    let mut post = method_states_and_behaviours_as_types::Post::new();

    post.add_text("Green green green!!");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("Green green green!!", post.content());
    println!("{}", post.content());
}
