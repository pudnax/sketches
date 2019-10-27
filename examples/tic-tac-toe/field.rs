use std::ops::Neg;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Field {
    Empty = 0,
    O = 1,
    X = 2,
}

impl Neg for Field {
    type Output = Field;

    fn neg(self) -> Self::Output {
        match self {
            Field::X => Field::O,
            Field::O => Field::X,
            Field::Empty => Field::Empty,
        }
    }
}

impl std::string::ToString for Field {
    fn to_string(&self) -> String {
        match self {
            Field::Empty => "".to_string(),
            Field::O => "O".to_string(),
            Field::X => "X".to_string(),
        }
    }
}
