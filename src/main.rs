extern crate chess;

use chess::*;

#[cfg(test)]
mod tests {
    use super::*;
    use rayon::prelude::*;
    fn perft(g: &ChessGame, depth: usize) -> u64 {
        if depth == 0 {
            return 1;
        }

        if depth > 2 {
            g.get_legal_moves(&g.turn)
                .par_iter()
                .map(|m| {
                    let mut b = g.clone();
                    b.apply_move(m);
                    b.switch_turn();

                    perft(&b, depth - 1)
                })
                .sum()
        } else {
            g.get_legal_moves(&g.turn)
                .iter()
                .map(|m| {
                    let mut b = g.clone();
                    b.apply_move(m);
                    b.switch_turn();

                    perft(&b, depth - 1)
                })
                .sum()
        }
    }

    #[test]
    fn perft_test() {
        assert_eq!(perft(&ChessGame::new(), 1), 20);
        assert_eq!(perft(&ChessGame::new(), 2), 400);
        assert_eq!(perft(&ChessGame::new(), 3), 8902);
        assert_eq!(perft(&ChessGame::new(), 4), 197281);
    }
}

fn print_board(board: &[ChessPiece; 64]) {
    use ChessColor::*;
    use ChessPiece::*;

    fn c(col: &ChessColor) -> String {
        return if *col == Wh {
            String::from("\x1b[34m")
        } else {
            String::from("\x1b[31m")
        };
    }

    for y in 0..8 {
        print!("{} ", 8 - y);
        for x in 0..8 {
            match &board[56 - y * 8 + x] {
                P(col) => print!("{}P\x1b[m", c(col)),
                R(col) => print!("{}R\x1b[m", c(col)),
                N(col) => print!("{}N\x1b[m", c(col)),
                B(col) => print!("{}B\x1b[m", c(col)),
                Q(col) => print!("{}Q\x1b[m", c(col)),
                K(col) => print!("{}K\x1b[m", c(col)),
                None => print!("."),
            };
        }
        println!();
    }
    println!("  abcdefgh");
}

fn main() {
    let g = ChessGame::new();

    let moves = g.get_legal_moves(&g.turn);

    for mv in moves {
        let mut cp = g.clone();

        cp.apply_move(&mv);

        print_board(cp.get_board());
    }
}
