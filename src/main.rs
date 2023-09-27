extern crate chess;

use chess::*;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::MouseButton;
use ggez::glam::*;
use ggez::graphics::{self, Color, DrawParam, PxScale, Rect};
use ggez::{event, GameError};
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
    selected_square: Option<Vec2>,
    move_squares: Vec<Vec2>,
    capture_squares: Vec<Vec2>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            boards: vec![ChessGame::new()],
            selected_square: None,
            move_squares: vec![],
            capture_squares: vec![],
        };
        Ok(s)
    }
}

impl event::EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));

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

        let mut draw_square = |i, j, col| {
            let square = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0., 0., window_height / 8.0, window_width / 8.0),
                col,
            )
            .unwrap();
            canvas.draw(
                &square,
                Vec2::new(
                    window_width / 8.0 * j as f32,
                    window_height / 8.0 * i as f32,
                ),
            );
        };

        for i in 0..8 {
            for j in 0..8 {
                let col = if (i + j) % 2 == 0 {
                    graphics::Color::from([0.2, 0.4, 0.6, 1.0])
                } else {
                    graphics::Color::from([0.1, 0.2, 0.3, 1.0])
                };

                draw_square(i, j, col);
            }
        }

        if let Some(highlighted_square) = self.selected_square {
            draw_square(
                highlighted_square.y as i32,
                highlighted_square.x as i32,
                graphics::Color::from([0.3, 0.6, 0.9, 1.0]),
            )
        }

        for pos in &self.move_squares {
            draw_square(
                pos.y as i32,
                pos.x as i32,
                graphics::Color::from([0.0, 1.0, 0.0, 0.3]),
            )
        }

        for pos in &self.capture_squares {
            draw_square(
                pos.y as i32,
                pos.x as i32,
                graphics::Color::from([1.0, 0.0, 0.0, 0.3]),
            )
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
                            x: window_width / 10.0,
                            y: window_height / 10.0,
                        });

                    let col = if p.color().unwrap() == ChessColor::Wh {
                        Color::from([0.8, 0.8, 0.8, 1.0])
                    } else {
                        Color::from([0.1, 0.1, 0.1, 1.0])
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

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        self.move_squares.clear();
        self.capture_squares.clear();
        let (window_width, _window_height) = ctx.gfx.drawable_size();

        let pos = Vec2::new(
            (x / (window_width / 8.)).floor(),
            (y / (window_width / 8.)).floor(),
        );

        let board = self.boards.last().unwrap();
        let moves = board.get_legal_moves(&board.turn);

        let mut add_move_squares = || {
            self.move_squares = moves
                .iter()
                .filter_map(|m| {
                    if m.origin == (7 - pos.y as usize) * 8 + pos.x as usize
                        && m.captures == ChessPiece::None
                    {
                        let x = m.target % 8;
                        let y = 7 - m.target / 8;
                        Some(Vec2::new(x as f32, y as f32))
                    } else {
                        None
                    }
                })
                .collect();
            self.capture_squares = moves
                .iter()
                .filter_map(|m| {
                    if m.origin == (7 - pos.y as usize) * 8 + pos.x as usize
                        && m.captures != ChessPiece::None
                    {
                        let x = m.target % 8;
                        let y = 7 - m.target / 8;
                        Some(Vec2::new(x as f32, y as f32))
                    } else {
                        None
                    }
                })
                .collect();
        };

        if self.selected_square.is_some_and(|p| p == pos) {
            self.selected_square = None;
        } else if self.selected_square.is_some() {
            let start_pos = Vec2::new(
                self.selected_square.unwrap().x,
                7. - self.selected_square.unwrap().y,
            );
            let end_pos = Vec2::new(pos.x, 7. - pos.y);
            self.selected_square = Some(pos);
            if let Some(m) = moves.iter().find(|m| {
                m.origin == start_pos.y as usize * 8 + start_pos.x as usize
                    && m.target == end_pos.y as usize * 8 + end_pos.x as usize
            }) {
                let mut b = board.clone();
                b.apply_move(m);
                b.switch_turn();
                self.boards.push(b);
            } else {
                if board.get_board()[(7 - pos.y as usize) * 8 + pos.x as usize] != ChessPiece::None
                {
                    add_move_squares();
                }
            }
        } else {
            self.selected_square = Some(pos);
            if board.get_board()[(7 - pos.y as usize) * 8 + pos.x as usize] != ChessPiece::None {
                add_move_squares();
            }
        }

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
