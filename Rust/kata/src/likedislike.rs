use std::ops::Add;

pub enum Action {
    Pressed(ButtonPressed),
    Nothing,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonPressed {
    #[allow(dead_code)]
    Like,
    #[allow(dead_code)]
    Dislike,
}

impl From<ButtonPressed> for Action {
    fn from(press: ButtonPressed) -> Self {
        Action::Pressed(press)
    }
}

impl Add for Action {
    type Output = Action;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Action::Pressed(v), Action::Pressed(w)) => (v == w)
                .then(|| Action::Nothing)
                .unwrap_or(Action::Pressed(w)),
            (Action::Pressed(v), Action::Nothing) | (Action::Nothing, Action::Pressed(v)) => {
                Action::Pressed(v)
            }
            (Action::Nothing, Action::Nothing) => Action::Nothing,
        }
    }
}

#[allow(dead_code)]
pub fn like_or_dislike(arr: Vec<ButtonPressed>) -> Action {
    arr.iter()
        .fold(Action::Nothing, |acc, &v| acc + Action::from(v))
}
