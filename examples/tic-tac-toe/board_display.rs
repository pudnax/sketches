use crate::board::{Board, BoardState};
use crate::field::Field;
use crate::player::Player;
use nannou::prelude::*;

impl Board {
    pub fn show_selections(&self, draw: &app::Draw, rect: &Rect) {
        let dims = (rect.right() - rect.left()) / 3.0;
        let draw_text = |sigil: &str, location: &Rect| {
            let text = text(sigil).font_size(dims as u32).build(*location);
            draw.path().fill().color(BLACK).events(text.path_events());
        };
        self.board.iter().enumerate().for_each(|(i, v)| {
            if *v != Field::Empty {
                match (i, v) {
                    (0, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.left() + dims / 2.0,
                            rect.top() - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    (1, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.right() / 3.0 - dims / 2.0,
                            rect.top() - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    (2, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.right() - dims / 2.0,
                            rect.top() - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    (3, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.left() + dims / 2.0,
                            rect.top() / 3.0 - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    (4, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.left() / 3.0 + dims / 2.0,
                            rect.top() / 3.0 - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    (5, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.right() - dims / 2.0,
                            rect.top() / 3.0 - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    (6, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.left() + dims / 2.0,
                            rect.bottom() / 3.0 - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    (7, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.right() / 3.0 - dims / 2.0,
                            rect.bottom() / 3.0 - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    (8, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.right() - dims / 2.0,
                            rect.bottom() / 3.0 - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    _ => unimplemented!(),
                };
            }
        });
    }
    fn show_winner(&self, draw: &app::Draw, rect: &Rect, win: (usize, usize, usize)) {
        let start_x;
        let start_y;
        let end_x;
        let end_y;
        let (top, _, bottom) = win;
        match (top, bottom) {
            // top row
            (0, 2) => {
                start_x = rect.left();
                start_y = rect.top() - rect.top() / 3.0;
                end_x = rect.right();
                end_y = rect.top() - rect.top() / 3.0;
            }
            // left column
            (0, 6) => {
                start_x = rect.left() - rect.left() / 3.0;
                start_y = rect.top();
                end_x = rect.left() - rect.left() / 3.0;
                end_y = rect.bottom();
            }
            // middle row
            (3, 5) => {
                start_x = rect.left();
                start_y = 0.0;
                end_x = rect.right();
                end_y = 0.0;
            }
            // middle column
            (1, 7) => {
                start_x = 0.0;
                start_y = rect.top();
                end_x = 0.0;
                end_y = rect.bottom();
            }
            // bottom row
            (6, 8) => {
                start_x = rect.left();
                start_y = rect.bottom() - rect.bottom() / 3.0;
                end_x = rect.right();
                end_y = rect.bottom() - rect.bottom() / 3.0;
            }
            // right column
            (2, 8) => {
                start_x = rect.right() - rect.right() / 3.0;
                start_y = rect.top();
                end_x = rect.right() - rect.right() / 3.0;
                end_y = rect.bottom();
            }
            // left diag
            (0, 8) => {
                start_x = rect.left();
                start_y = rect.top();
                end_x = rect.right();
                end_y = rect.bottom();
            }
            // right diag
            (2, 6) => {
                start_x = rect.right();
                start_y = rect.top();
                end_x = rect.left();
                end_y = rect.bottom();
            }
            _ => unreachable!(),
        }
        draw.line()
            .start(pt2(start_x, start_y))
            .end(pt2(end_x, end_y))
            .stroke_weight(2.0)
            .color(BLACK);
    }
    pub fn display(&self, draw: &app::Draw, rect: &Rect) {
        match &self.state {
            BoardState::Tie => {
                self.show_selections(draw, rect);
                let location = rect.pad(20.0);
                let wins = "Tie!".to_string();
                let text = text(&wins).font_size(75).build(location);
                draw.path().fill().color(WHITE).events(text.path_events());
            }
            BoardState::InGame => {
                self.show_selections(draw, rect);
            }
            winner => {
                self.show_selections(draw, rect);
                let (winning_player, winning_pos) = match winner {
                    BoardState::Winner(Player::Human, x) => (Field::X, x),
                    BoardState::Winner(Player::Computer, x) => (Field::O, x),
                    _ => unreachable!(),
                };
                self.show_winner(draw, rect, *winning_pos);
                let location = rect.pad(20.0);
                let wins = format!("{} Wins!", &winning_player.to_string());
                let text = text(&wins).font_size(75).build(location);
                draw.path().fill().color(WHITE).events(text.path_events());
            }
        }
    }
}
