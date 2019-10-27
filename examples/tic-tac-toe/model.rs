use crate::board::{Board, BoardState};
use nannou::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum GameMode {
    SinglePlayer,
    MultiPlayer,
    Waiting,
}

#[derive(Debug)]
pub struct Model {
    pub board: Board,
    pub mode: GameMode,
}

impl Model {
    pub fn check_new(&mut self, app: &App) {
        if self.board.state != BoardState::InGame {
            self.mode = GameMode::Waiting;
            self.board = Board::new(app.window_rect());
        } else {
            self.board.register_click(&app);
        }
    }
    pub fn check_mode(&self, rect: &Rect, mouse: Point2) -> GameMode {
        let x_single_player = rect.left() / 3.0;
        let y_single_player = 0.0;
        let x_multi_player = rect.right() / 3.0;
        let y_multi_player = 0.0;
        let width = 150.0;
        let height = width / 1.618;
        let single_player =
            Rect::from_x_y_w_h(x_single_player, y_single_player, width, height);
        let multi_player =
            Rect::from_x_y_w_h(x_multi_player, y_multi_player, width, height);

        if single_player.contains(mouse) {
            return GameMode::SinglePlayer;
        } else if multi_player.contains(mouse) {
            return GameMode::MultiPlayer;
        };
        GameMode::Waiting
    }
    pub fn display(&self, draw: &app::Draw, rect: &Rect) {
        match self.mode {
            GameMode::Waiting => {
                let x_single_player = rect.left() / 3.0;
                let y_single_player = 0.0;
                let x_multi_player = rect.right() / 3.0;
                let y_multi_player = 0.0;
                let width = 150.0;
                let height = width / 1.618;

                draw.rect()
                    .x_y(x_single_player, y_single_player)
                    .w_h(width, height)
                    .color(DARKGREY);
                draw.rect()
                    .x_y(x_multi_player, y_multi_player)
                    .w_h(width, height)
                    .color(DARKGREY);
                let single_player =
                    Rect::from_x_y_w_h(x_single_player, y_single_player, width, height);
                let multi_player =
                    Rect::from_x_y_w_h(x_multi_player, y_multi_player, width, height);

                let spt = text("Single Player").font_size(20).build(single_player);
                draw.path().fill().color(BLACK).events(spt.path_events());
                let mpt = text("Mutliplayer").font_size(20).build(multi_player);
                draw.path().fill().color(BLACK).events(mpt.path_events());
            }
            _ => {
                // right vertical line
                draw.line()
                    .start(pt2(rect.right() / 3.0, rect.top()))
                    .end(pt2(rect.right() / 3.0, rect.bottom()))
                    .stroke_weight(2.0)
                    .color(DARKGREY);
                // left vertical line
                draw.line()
                    .start(pt2(rect.left() / 3.0, rect.top()))
                    .end(pt2(rect.left() / 3.0, rect.bottom()))
                    .stroke_weight(2.0)
                    .color(DARKGREY);
                // bottom horizontal line
                draw.line()
                    .start(pt2(rect.left(), rect.bottom() / 3.0))
                    .end(pt2(rect.right(), rect.bottom() / 3.0))
                    .stroke_weight(2.0)
                    .color(DARKGREY);
                // top horizontal line
                draw.line()
                    .start(pt2(rect.left(), rect.top() / 3.0))
                    .end(pt2(rect.right(), rect.top() / 3.0))
                    .stroke_weight(2.0)
                    .color(DARKGREY);
                self.board.display(draw, &rect);
            }
        }
    }
}
