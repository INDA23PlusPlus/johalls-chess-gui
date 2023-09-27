extern crate chess;

use chess::*;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::glam::*;
use ggez::graphics::{self, Color, DrawParam, PxScale, Rect};
use ggez::{Context, GameResult};

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

struct MainState {
    boards: Vec<ChessGame>,
    pos_x: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            boards: vec![ChessGame::new()],
            pos_x: 0.0,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        // let circle = graphics::Mesh::new_circle(
        //     ctx,
        //     graphics::DrawMode::fill(),
        //     Vec2::new(0.0, 0.0),
        //     100.0,
        //     2.0,
        //     Color::WHITE,
        // )?;
        // canvas.draw(&circle, Vec2::new(self.pos_x, 380.0));
        let (window_width, window_height) = ctx.gfx.drawable_size();
        for i in 0..8 {
            for j in 0..8 {
                let col = if (i + j) % 2 == 0 {
                    graphics::Color::from([0.2, 0.4, 0.6, 1.0])
                } else {
                    graphics::Color::from([0.1, 0.2, 0.3, 1.0])
                };

                let square = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    Rect::new(0., 0., window_height / 8.0, window_width / 8.0),
                    col,
                )?;
                canvas.draw(
                    &square,
                    Vec2::new(
                        window_width / 8.0 * i as f32,
                        window_height / 8.0 * j as f32,
                    ),
                );
            }
        }

        let board = self.boards.last().unwrap();

        for i in 0..8 {
            for j in 0..8 {
                let p = board.get_board()[i * 8 + j];

                if p != ChessPiece::None {
                    let mut t = graphics::Text::new(match p {
                        ChessPiece::P(_) => "P",
                        ChessPiece::N(_) => "N",
                        ChessPiece::B(_) => "B",
                        ChessPiece::R(_) => "R",
                        ChessPiece::Q(_) => "Q",
                        ChessPiece::K(_) => "K",
                        ChessPiece::None => " ",
                    });
                    t.set_bounds(Vec2::new(window_width / 10., window_height / 10.))
                        .set_scale(PxScale {
                            x: window_width / 10.,
                            y: window_height / 10.,
                        });

                    let col = if p.color().unwrap() == ChessColor::Wh {
                        Color::WHITE
                    } else {
                        Color::BLACK
                    };
                    canvas.draw(
                        &t,
                        DrawParam::default()
                            .dest(Vec2::new(
                                window_width / 8.0 * j as f32,
                                window_height / 8.0 * (7 - i) as f32,
                            ))
                            .color(col),
                    );
                }
            }
        }

        // canvas.draw(&graphics::Image::from_path(ctx, "/icon.png")?, Vec2::new(0., 0.));

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Chess", "JonathanHallstrom")
        .window_mode(WindowMode::default().dimensions(800., 800.))
        .window_setup(WindowSetup::default().title("Chess").icon("/icon.png"));
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}

