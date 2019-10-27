use crate::board::BoardState;
use crate::eval::Eval;
use crate::field::Field;
use crate::player::Player;

pub fn minimax(state: BoardState, board: &[Field], player: Player, depth: i64) -> Eval {
    match state {
        BoardState::Tie => Eval {
            position: 0,
            score: 0,
        },
        BoardState::InGame => {
            let evaluated_moves: Vec<Eval> = board
                .iter()
                .enumerate()
                .filter_map(|(i, v)| match v {
                    Field::Empty => {
                        let mut cloned_board = Vec::from(board).clone();
                        let new_field = match player {
                            Player::Human => Field::X,
                            Player::Computer => Field::O,
                        };
                        cloned_board[i] = new_field;
                        let score = minimax(
                            check_winner(&cloned_board),
                            &cloned_board,
                            -player,
                            depth + 1,
                        )
                        .score;
                        Some(Eval::new(i, score))
                    }
                    _ => None,
                })
                .collect();
            let mut cloned_evals = evaluated_moves.clone();
            cloned_evals.sort();
            match player {
                Player::Human => {
                    let last = cloned_evals.last();
                    *last.unwrap()
                }
                Player::Computer => {
                    let first = cloned_evals.first();
                    *first.unwrap()
                }
            }
        }
        winner => match winner {
            BoardState::Winner(Player::Human, _) => Eval {
                position: 0,
                score: depth - 10,
            },
            BoardState::Winner(Player::Computer, _x) => Eval {
                position: 0,
                score: 10 - depth,
            },
            _ => unreachable!(),
        },
    }
}
pub fn check_winner(board: &[Field]) -> BoardState {
    let winning_boards = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],
        vec![0, 3, 6],
        vec![1, 4, 7],
        vec![2, 5, 8],
        vec![0, 4, 8],
        vec![2, 4, 6],
    ];
    let mut winning = None;
    winning_boards.iter().any(|ts| {
        if board[ts[0]] == board[ts[1]] && board[ts[1]] == board[ts[2]] {
            if board[ts[0]] == Field::X {
                winning = Some(BoardState::Winner(Player::Human, (ts[0], ts[1], ts[2])));
                return true;
            } else if board[ts[0]] == Field::O {
                winning =
                    Some(BoardState::Winner(Player::Computer, (ts[0], ts[1], ts[2])));
                return true;
            }
        }
        false
    });

    if !board.contains(&Field::Empty) && winning.is_none() {
        winning = Some(BoardState::Tie);
    }
    winning.unwrap_or_else(|| BoardState::InGame)
}

mod test {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use crate::board::Board;
    #[allow(unused_imports)]
    use nannou::geom::Rect;
    #[test]
    fn top_row_human_win() {
        let board = &[
            Field::X,
            Field::X,
            Field::X,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
        ];
        let actual = check_winner(board);
        assert_eq!(
            actual,
            BoardState::Winner(Player::Human, (0, 1, 2)),
            "Human Win"
        );
    }
    #[test]
    fn mid_row_human_win() {
        let board = &[
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::X,
            Field::X,
            Field::X,
            Field::Empty,
            Field::Empty,
            Field::Empty,
        ];
        let actual = check_winner(board);
        assert_eq!(
            actual,
            BoardState::Winner(Player::Human, (3, 4, 5)),
            "Human Win"
        );
    }
    #[test]
    fn bot_row_human_win() {
        let board = &[
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::X,
            Field::X,
            Field::X,
        ];
        let actual = check_winner(board);
        assert_eq!(
            actual,
            BoardState::Winner(Player::Human, (6, 7, 8)),
            "Human Win"
        );
    }
    #[test]
    fn top_row_comp_win() {
        let board = &[
            Field::O,
            Field::O,
            Field::O,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
        ];
        let actual = check_winner(board);
        assert_eq!(
            actual,
            BoardState::Winner(Player::Computer, (0, 1, 2)),
            "Computer Win"
        );
    }
    #[test]
    fn mid_row_comp_win() {
        let board = &[
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::O,
            Field::O,
            Field::O,
            Field::Empty,
            Field::Empty,
            Field::Empty,
        ];
        let actual = check_winner(board);
        assert_eq!(
            actual,
            BoardState::Winner(Player::Computer, (3, 4, 5)),
            "Computer Win"
        );
    }
    #[test]
    fn bot_row_comp_win() {
        let board = &[
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::O,
            Field::O,
            Field::O,
        ];
        let actual = check_winner(board);
        assert_eq!(
            actual,
            BoardState::Winner(Player::Computer, (6, 7, 8)),
            "Computer Win"
        );
    }
    #[test]
    fn left_col_human_win() {
        let board = &[
            Field::X,
            Field::Empty,
            Field::Empty,
            Field::X,
            Field::Empty,
            Field::Empty,
            Field::X,
            Field::Empty,
            Field::Empty,
        ];
        let actual = check_winner(board);
        assert_eq!(actual, BoardState::Winner(Player::Human, (0, 3, 6)),);
    }
    #[test]
    fn mid_col_human_win() {
        let board = &[
            Field::Empty,
            Field::X,
            Field::Empty,
            Field::Empty,
            Field::X,
            Field::Empty,
            Field::Empty,
            Field::X,
            Field::Empty,
        ];
        let actual = check_winner(board);
        assert_eq!(actual, BoardState::Winner(Player::Human, (1, 4, 7)),);
    }
    #[test]
    fn right_col_human_win() {
        let board = &[
            Field::Empty,
            Field::Empty,
            Field::X,
            Field::Empty,
            Field::Empty,
            Field::X,
            Field::Empty,
            Field::Empty,
            Field::X,
        ];
        let actual = check_winner(board);
        assert_eq!(actual, BoardState::Winner(Player::Human, (2, 5, 8)),);
    }
    #[test]
    fn left_col_comp_win() {
        let board = &[
            Field::O,
            Field::Empty,
            Field::Empty,
            Field::O,
            Field::Empty,
            Field::Empty,
            Field::O,
            Field::Empty,
            Field::Empty,
        ];
        let actual = check_winner(board);
        assert_eq!(actual, BoardState::Winner(Player::Computer, (0, 3, 6)),);
    }
    #[test]
    fn mid_col_comp_win() {
        let board = &[
            Field::Empty,
            Field::O,
            Field::Empty,
            Field::Empty,
            Field::O,
            Field::Empty,
            Field::Empty,
            Field::O,
            Field::Empty,
        ];
        let actual = check_winner(board);
        assert_eq!(actual, BoardState::Winner(Player::Computer, (1, 4, 7)),);
    }
    #[test]
    fn right_col_comp_win() {
        let board = &[
            Field::Empty,
            Field::Empty,
            Field::O,
            Field::Empty,
            Field::Empty,
            Field::O,
            Field::Empty,
            Field::Empty,
            Field::O,
        ];
        let actual = check_winner(board);
        assert_eq!(actual, BoardState::Winner(Player::Computer, (2, 5, 8)),);
    }
    #[test]
    fn left_diag_human_win() {
        let board = &[
            Field::X,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::X,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::X,
        ];
        let actual = check_winner(board);
        assert_eq!(actual, BoardState::Winner(Player::Human, (0, 4, 8)),);
    }
    #[test]
    fn right_diag_human_win() {
        let board = &[
            Field::Empty,
            Field::Empty,
            Field::X,
            Field::Empty,
            Field::X,
            Field::Empty,
            Field::X,
            Field::Empty,
            Field::Empty,
        ];
        let actual = check_winner(board);
        assert_eq!(actual, BoardState::Winner(Player::Human, (2, 4, 6)),);
    }
    #[test]
    fn left_diag_comp_win() {
        let board = &[
            Field::O,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::O,
            Field::Empty,
            Field::Empty,
            Field::Empty,
            Field::O,
        ];
        let actual = check_winner(board);
        assert_eq!(actual, BoardState::Winner(Player::Computer, (0, 4, 8)),);
    }
    #[test]
    fn right_diag_comp_win() {
        let board = &[
            Field::Empty,
            Field::Empty,
            Field::O,
            Field::Empty,
            Field::O,
            Field::Empty,
            Field::O,
            Field::Empty,
            Field::Empty,
        ];
        let actual = check_winner(board);
        assert_eq!(actual, BoardState::Winner(Player::Computer, (2, 4, 6)),);
    }
    #[test]
    fn computer_optimal_play() {
        let mut field = Field::X;
        let mut game = Board::new(Rect::from_w_h(800.0, 800.0));
        let mut winner = None;
        (0..90).any(|_| {
            let eval = minimax(game.state, &game.board, game.current_player, 0);
            game.board[eval.position] = field;
            game.made_move();
            match game.state {
                BoardState::InGame => {
                    field = -field;
                    return false;
                }
                BoardState::Tie => {
                    // game.board = (0..9).map(|_| Field::Empty).collect();
                    let new_game = Board::new(game.rect);
                    std::mem::replace(&mut game, new_game);
                    return false;
                }
                _ => {
                    winner = Some("winner");
                    return true;
                }
            }
        });
        assert!(winner.is_none());
    }
}
