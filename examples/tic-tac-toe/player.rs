#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Player {
    Human = 1,
    Computer = -1,
}

impl Default for Player {
    fn default() -> Self {
        Player::Human
    }
}

impl std::ops::Neg for Player {
    type Output = Player;

    fn neg(self) -> Self::Output {
        match self {
            Player::Human => Player::Computer,
            Player::Computer => Player::Human,
        }
    }
}
