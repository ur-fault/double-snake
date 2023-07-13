use std::time::Duration;

use crossterm::style::{Color, ContentStyle, Stylize};
use rand::{seq::SliceRandom, thread_rng};
use terminal_renderer::{
    canvas::CanvasLike, cell::Cell, drawable::Drawable, frame::Frame, renderer::Dims,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BoardState {
    #[default]
    Running,
    Won {
        is_player2: bool,
    },
    BothLost,
}

use crate::{
    helpers::value_if_else,
    ui::{dims::IntoUidims, uibox::UiBox},
};
pub struct Board {
    snake1: Snake,
    snake2: Snake,
    food: Dims,
    state: BoardState,
}

const TELEPORT_ON_EDGE: bool = true;

impl Board {
    pub fn new() -> Self {
        let snakes = [
            Snake::new((0, 0), Color::Green),
            Snake::new((7, 7), Color::Blue),
        ];
        let [s1, s2] = snakes;

        let mut board = Self {
            snake1: s1,
            snake2: s2,
            food: (0, 0),
            state: BoardState::default(),
        };
        board.move_food();

        board
    }

    pub fn update(&mut self, snake1_dir: Option<Dir>, snake2_dir: Option<Dir>) {
        if self.state != BoardState::Running {
            return;
        }

        self.snake1.go(snake1_dir);
        self.snake2.go(snake2_dir);

        if self.snake1.pos() == self.food {
            self.snake1.body.push(self.snake1.last_end);
            self.move_food();
        }

        if self.snake2.pos() == self.food {
            self.snake2.body.push(self.snake2.last_end);
            self.move_food();
        }

        if self.snake1.pos() == self.snake2.pos() {
            self.state = BoardState::BothLost;
            return;
        }

        if self.snake2.body.contains(&self.snake1.pos())
            || self.snake1.tail().contains(&self.snake1.pos())
        {
            self.state = BoardState::Won { is_player2: true };
            return;
        }

        if self.snake1.body.contains(&self.snake2.pos())
            || self.snake2.tail().contains(&self.snake2.pos())
        {
            self.state = BoardState::Won { is_player2: false };
            return;
        }
    }

    fn random_food_pos(&self) -> Dims {
        (0..8)
            .map(|x| (0..8).map(|y| (x, y)).collect::<Vec<_>>())
            .flatten()
            .filter(|pos| !self.snake1.body.contains(pos) && !self.snake2.body.contains(pos))
            .collect::<Vec<_>>()
            .choose(&mut thread_rng())
            .unwrap()
            .clone()
    }

    fn move_food(&mut self) {
        self.food = self.random_food_pos();
    }

    pub fn cell_size(&self) -> Dims {
        (6, 3)
    }

    pub fn border_size(&self) -> Dims {
        (2, 1)
    }

    pub fn render_size(&self) -> Dims {
        (
            8 * self.cell_size().0 + 7 * self.border_size().0 + 2,
            8 * self.cell_size().1 + 7 * self.border_size().1 + 2,
        )
    }

    pub fn get_update_intervar() -> Duration {
        Duration::from_millis(300)
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn get_state(&self) -> BoardState {
        self.state
    }

    pub fn get_scores(&self) -> (usize, usize) {
        (self.snake1.body.len(), self.snake2.body.len())
    }
}

impl Drawable for Board {
    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
        let size = self.render_size();

        UiBox::new(size.into(), ContentStyle::new()).draw(pos, frame);
        let inner = Frame::new(frame).with_pos(pos).with_size(size).mx(1).my(1);
        let make_cell_frame = |pos: Dims| {
            Frame::new(inner.clone())
                .with_pos(
                    ((self.cell_size().into_ui() + self.border_size().into_ui()) * pos.into_ui())
                        .into(),
                )
                .with_size(self.cell_size())
        };

        for x in 0..7 {
            Frame::new(inner.clone()) // cuz clip
                .l((self.cell_size().0 + self.border_size().0) * (x + 1))
                .r(self.border_size().0)
                .fill(Cell::styled(' ', ContentStyle::new().on_white()));
        }

        for y in 0..7 {
            Frame::new(inner.clone()) // cuz clip
                .t((self.cell_size().1 + self.border_size().1) * (y + 1))
                .b(self.border_size().1)
                .fill(Cell::styled(' ', ContentStyle::new().on_white()));
        }

        match self.state {
            BoardState::Running => {
                for snake in [&self.snake1, &self.snake2] {
                    for block in snake.body.iter() {
                        make_cell_frame(*block)
                            .fill(Cell::styled(' ', ContentStyle::new().on(snake.color)));
                    }
                }

                make_cell_frame(self.food).fill(Cell::styled(' ', ContentStyle::new().on_yellow()));
            }
            BoardState::Won { is_player2 } => {
                for x in 0..8 {
                    for y in 0..8 {
                        make_cell_frame((x, y)).fill(Cell::styled(
                            ' ',
                            ContentStyle::new().on(value_if_else(
                                is_player2,
                                || self.snake2.color,
                                || self.snake1.color,
                            )),
                        ));
                    }
                }
            }
            BoardState::BothLost => {
                for x in 0..8 {
                    for y in 0..8 {
                        make_cell_frame((x, y))
                            .fill(Cell::styled(' ', ContentStyle::new().on(Color::Red)));
                    }
                }
                for x in 1..7 {
                    make_cell_frame((x, x))
                        .fill(Cell::styled(' ', ContentStyle::new().on(Color::Black)));
                    make_cell_frame((x, 7 - x))
                        .fill(Cell::styled(' ', ContentStyle::new().on(Color::Black)));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    body: Vec<Dims>,
    color: Color,
    dir: Dir,
    last_end: Dims,
}

impl Snake {
    fn is_on_valid_pos(pos: Dims) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < 8 && pos.1 < 8
    }

    fn new(pos: Dims, color: Color) -> Self {
        if !Self::is_on_valid_pos(pos) {
            panic!("Invalid position");
        }
        Self {
            body: vec![pos],
            color,
            dir: Dir::Right,
            last_end: pos,
        }
    }

    fn pos(&self) -> Dims {
        self.body[0]
    }

    fn tail(&self) -> &[Dims] {
        &self.body[1..]
    }

    fn go(&mut self, dir: Option<Dir>) {
        if let Some(dir) = dir {
            self.dir = dir;
        }

        let mut new_head = self.body[0];
        match self.dir {
            Dir::Up => new_head.1 -= 1,
            Dir::Down => new_head.1 += 1,
            Dir::Left => new_head.0 -= 1,
            Dir::Right => new_head.0 += 1,
        }

        if !Self::is_on_valid_pos(new_head) {
            if TELEPORT_ON_EDGE {
                if new_head.0 < 0 {
                    new_head.0 = 7;
                } else if new_head.0 > 7 {
                    new_head.0 = 0;
                } else if new_head.1 < 0 {
                    new_head.1 = 7;
                } else if new_head.1 > 7 {
                    new_head.1 = 0;
                }
            } else {
                panic!("Invalid position");
            }
        }

        self.body.insert(0, new_head);
        self.last_end = self.body.pop().unwrap();
    }
}
