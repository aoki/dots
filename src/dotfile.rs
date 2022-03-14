#[derive(Debug)]
enum State {
    Linked,
    Unliked,
    Other,
}

#[derive(Debug)]
pub struct Dotfile {
    from: String,
    to: String,
    state: State,
}
