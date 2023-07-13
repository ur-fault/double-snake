use std::{
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, KeyCode, KeyModifiers},
    style::{ContentStyle, Stylize},
};
use double_snake::board::{Board, BoardState, Dir};
use terminal_renderer::{
    drawable::{misc::RightAlignedStringExt, Drawable},
    frame::Frame,
    renderer::Renderer,
};

#[derive(Clone, Copy, Default)]
struct PlayerButtons(bool, bool);

struct GameInput {
    snake1_dir: Option<Dir>,
    snake2_dir: Option<Dir>,
    p1: PlayerButtons,
    p2: PlayerButtons,
}

impl GameInput {
    fn new() -> Self {
        Self {
            snake1_dir: None,
            snake2_dir: None,
            p1: PlayerButtons(false, false),
            p2: PlayerButtons(false, false),
        }
    }

    fn process_available_events(&mut self, mut event_callback: impl FnMut(event::Event)) {
        *self = Self::new(); // Reset the input

        while let Ok(true) = event::poll(Duration::from_nanos(1)) {
            let event = event::read().unwrap();
            match event {
                event::Event::Key(event::KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: event::KeyEventKind::Press,
                    ..
                }) => panic!("CTRL+C pressed"),

                event::Event::Key(event::KeyEvent {
                    code,
                    kind: event::KeyEventKind::Press | event::KeyEventKind::Repeat,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    use KeyCode::*;
                    match code {
                        Char('w' | 'W') => self.snake1_dir = Some(Dir::Up),
                        Char('a' | 'A') => self.snake1_dir = Some(Dir::Left),
                        Char('s' | 'S') => self.snake1_dir = Some(Dir::Down),
                        Char('d' | 'D') => self.snake1_dir = Some(Dir::Right),
                        Up => self.snake2_dir = Some(Dir::Up),
                        Left => self.snake2_dir = Some(Dir::Left),
                        Down => self.snake2_dir = Some(Dir::Down),
                        Right => self.snake2_dir = Some(Dir::Right),
                        Char('r' | 'R') => self.p1.0 = true,
                        Char('f' | 'F') => self.p1.1 = true,
                        Char('5') => self.p2.0 = true,
                        Char('2') => self.p2.1 = true,
                        _ => {}
                    }
                }
                _ => {}
            }
            event_callback(event);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = Renderer::new()?;
    let mut render_space = renderer.get_render_space();

    let mut board = Board::new();

    let mut last_frame_start = Instant::now();
    let mut input = GameInput::new();

    loop {
        // Sleep until the next frame should start
        let now = Instant::now();
        let delta = now - last_frame_start;
        thread::sleep(
            Board::get_update_intervar()
                .checked_sub(delta)
                .unwrap_or_default(),
        );
        last_frame_start = Instant::now();

        input.process_available_events(|e| renderer.on_event(&e).unwrap());
        if input.p1.1 || input.p2.1 {
            break;
        }

        if board.get_state() != BoardState::Running && (input.p1.0 || input.p2.0) {
            board.reset();
            continue;
        }

        board.update(input.snake1_dir, input.snake2_dir);

        "P1: WASD, confirm: R, back: F; P2: Arrows, confirm: 5, back: 2"
            .draw((1, 0), &mut render_space);
        (
            format!("P1 Score: {}", board.get_scores().0),
            ContentStyle::new().green(),
        )
            .draw((1, 1), &mut render_space);
        (
            format!("P2 Score: {}", board.get_scores().1).right(),
            ContentStyle::new().blue(),
        )
            .draw(
                (1, 1),
                &mut Frame::new(&mut render_space).l(board.render_size().0).mx(1),
            );
        board.draw((0, 2).into(), &mut render_space);

        renderer.render()?;
    }

    Ok(())
}
