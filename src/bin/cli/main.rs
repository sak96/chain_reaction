use chain_reaction::board::{Board, BoardState};

fn get_input(player: u8) -> (usize, usize) {
    let mut input = String::new();
    loop {
        println!("{} input:", player);
        std::io::stdin()
            .read_line(&mut input)
            .expect("Not a valid string");
        if let Some((xin, yin)) = input.split_once(' ') {
            if let Ok(x) = xin.trim().parse() {
                if let Ok(y) = yin.trim().parse() {
                    return (x, y);
                }
            }
        }
        println!("parsing failed {}", input);
        input.clear();
    }
}

fn main() {
    let mut board = Board::new(10, 10, 2);
    let player = loop {
        println!("{}", board);
        match board.state() {
            BoardState::GameOver(player) => {
                break player;
            }
            BoardState::Wait => {
                let cur_player = board.current_player_id();
                let (x, y) = get_input(cur_player);
                if let Err(x) = board.player_move(cur_player, x, y) {
                    println!("{:?}", x);
                }
            }
            _ => {
                board.next_iteration();
            }
        }
    };
    println!("{} player won", player);
}

