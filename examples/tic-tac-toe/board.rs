use crate::brain;
use crate::player::Player;
use crate::Field;
use nannou::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BoardState {
    Winner(Player, (usize, usize, usize)),
    Tie,
    InGame,
}

#[derive(Debug)]
pub struct Board {
    pub rect: geom::Rect,
    pub board: Vec<Field>,
    pub player_1: Player,
    pub player_2: Player,
    pub current_player: Player,
    pub state: BoardState,
}

impl Board {
    pub fn new(rect: geom::Rect) -> Self {
        Board {
            rect,
            board: (0..9).map(|_| Field::Empty).collect(),
            player_1: Player::Human,
            player_2: Player::Computer,
            current_player: Player::Human,
            state: BoardState::InGame,
        }
    }
    pub fn computer_move(&mut self) {
        if self.state == BoardState::InGame && self.current_player == Player::Computer {
            let eval = brain::minimax(self.state, &self.board, self.current_player, 0);
            self.board[eval.position] = Field::O;
            self.made_move();
        }
    }

    pub fn made_move(&mut self) {
        self.state = brain::check_winner(&self.board);
        self.current_player = -self.current_player;
    }

    fn left_column(&self, mouse_x: f32) -> bool {
        mouse_x < self.rect.left() / 3.0
    }
    fn right_column(&self, mouse_x: f32) -> bool {
        mouse_x > self.rect.right() / 3.0
    }
    fn center_column(&self, mouse_x: f32) -> bool {
        !self.left_column(mouse_x) && !self.right_column(mouse_x)
    }
    fn top_row(&self, mouse_y: f32) -> bool {
        mouse_y > self.rect.top() / 3.0
    }
    fn bottom_row(&self, mouse_y: f32) -> bool {
        mouse_y < self.rect.bottom() / 3.0
    }
    fn center_row(&self, mouse_y: f32) -> bool {
        !self.top_row(mouse_y) && !self.bottom_row(mouse_y)
    }
    pub fn register_click(&mut self, app: &App) {
        match self.state {
            BoardState::InGame => {
                let (x, y) = (app.mouse.position().x, app.mouse.position().y);
                let location = match (x, y) {
                    (x, y) if self.left_column(x) && self.top_row(y) => 0,
                    (x, y) if self.center_column(x) && self.top_row(y) => 1,
                    (x, y) if self.right_column(x) && self.top_row(y) => 2,

                    (x, y) if self.left_column(x) && self.center_row(y) => 3,
                    (x, y) if self.center_column(x) && self.center_row(y) => 4,
                    (x, y) if self.right_column(x) && self.center_row(y) => 5,

                    (x, y) if self.left_column(x) && self.bottom_row(y) => 6,
                    (x, y) if self.center_column(x) && self.bottom_row(y) => 7,
                    (x, y) if self.right_column(x) && self.bottom_row(y) => 8,

                    _ => unreachable!(),
                };
                if self.board[location] != Field::Empty {
                    return;
                }
                match self.current_player {
                    Player::Human => self.board[location] = Field::X,
                    Player::Computer => self.board[location] = Field::O,
                }
                self.made_move();
            }
            _ => {
                let new_self = Self::new(self.rect);
                std::mem::replace(self, new_self);
            }
        };
    }
}
