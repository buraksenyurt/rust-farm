fn main() {
    let mut state = State::Normal;

    let mut expression = String::new();
    let input = "Select _*_ from ^Products^. #Tablodan veri çekme operasyonu#";

    for c in input.chars() {
        let (o, new_state) = engine(state, c);

        if let Some(c) = o {
            expression.push(c);
        }

        state = new_state;
    }

    println!("{}", expression);
}

#[derive(Copy, Clone)]
enum State {
    Normal,
    Comment,
    Upper,
    Lower,
}

fn engine(state: State, c: char) -> (Option<char>, State) {
    use self::State::*;
    match (state, c) {
        (Normal, '#') => (None, Comment),
        (Normal, '^') => (None, Upper),
        (Normal, '_') => (None, Lower),
        (Normal, other) => (Some(other), Normal),
        (Comment, '#') => (None, Normal),
        (Comment, _) => (None, Comment),
        (Upper, '^') => (None, Normal),
        (Upper, other) => (Some(other.to_ascii_uppercase()), Upper),
        (Lower, '_') => (None, Normal),
        (Lower, other) => (Some(other.to_ascii_lowercase()), Lower),
    }
}
