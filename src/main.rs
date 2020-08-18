mod method_state_pattern;
mod method_states_and_behaviours_as_types;

fn main() {
    let mut post0 = method_state_pattern::Post::new();

    post0.add_text("Go green!");
    assert_eq!("", post0.content());

    post0.request_review();
    assert_eq!("", post0.content());

    post0.approve();
    assert_eq!("Go green!", post0.content());
    println!("{}", post0.content());


    let mut post1 = method_states_and_behaviours_as_types::Post::new();

    post1.add_text("Green green green!!");

    let post1 = post1.request_review();

    let post1 = post1.approve();

    assert_eq!("Green green green!!", post1.content());
    println!("{}", post1.content());
}
