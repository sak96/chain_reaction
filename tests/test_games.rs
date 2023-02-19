use chain_reaction::board::{Board, BoardState};

/// player, x, y, iteration
#[derive(Debug)]
struct Move(u8, usize, usize, usize);

/// Game describes one game
#[derive(Debug)]
struct Game {
    winner: Option<u8>,
    moves: Vec<Move>,
    rows: usize,
    cols: usize,
    players: u8,
}

fn assert_game(game: Game) {
    let mut board = Board::new(game.rows, game.cols, game.players);
    for Move(player, row, col, iter) in game.moves {
        let result = board.player_move(player, row, col);
        assert!(result.is_ok(), "{:?}", result);
        let mut iter_ = 1;
        while board.next_iteration() {
            iter_ += 1;
        }
        assert_eq!(iter, iter_);
    }
    if let Some(w) = game.winner {
        assert_eq!(board.state(), &BoardState::GameOver(w));
    }
}

#[test]
fn short_game() {
    let game = Game {
        winner: Some(0),
        moves: vec![Move(0, 0, 0, 1), Move(1, 1, 0, 1), Move(0, 0, 0, 2)],
        rows: 5,
        cols: 5,
        players: 2,
    };
    assert_game(game);
}
