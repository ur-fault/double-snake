use crossterm::style::{Color, ContentStyle};
pub use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    terminal::size,
};

use pad::PadStr;
use terminal_renderer::{
    canvas::CanvasLike,
    drawable::Drawable,
    frame::Frame,
    renderer::{Dims, Renderer},
};

use crate::helpers::is_key_release;

use super::{
    uibox::{helpers::box_center, UiBox},
    *,
};

#[derive(Debug)]
pub enum MenuError {
    CrosstermError(CrosstermError),
    EmptyMenu,
    Exit,
    FullQuit,
}

impl From<CrosstermError> for MenuError {
    fn from(error: CrosstermError) -> Self {
        Self::CrosstermError(error)
    }
}

impl From<crossterm::ErrorKind> for MenuError {
    fn from(error: crossterm::ErrorKind) -> Self {
        Self::CrosstermError(error.try_into().expect("Cannot convert crossterm error"))
    }
}

pub fn menu(
    renderer: &mut Renderer,
    frame: &mut impl CanvasLike,
    box_style: ContentStyle,
    text_style: ContentStyle,
    title: &str,
    options: &[&str],
    default: Option<usize>,
    counted: bool,
) -> Result<u16, MenuError> {
    let mut selected = default.unwrap_or(0);
    let opt_count = options.len();

    if opt_count == 0 {
        return Err(MenuError::EmptyMenu);
    }

    let mut menu_drawable = Menu {
        title: title,
        options: options,
        counted: counted,
        selected: selected,
        default,
        box_style,
        text_style,
    };
    let pos = box_center(frame.size().into(), menu_drawable.size().into()).into();

    menu_drawable.draw(pos, frame);
    renderer.render()?;

    loop {
        let event = read()?;

        match event {
            Event::Key(KeyEvent { code, kind, .. }) if !is_key_release(kind) => match code {
                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                    selected = if selected == 0 {
                        opt_count - 1
                    } else {
                        selected - 1
                    }
                }
                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                    selected = (selected + 1) % opt_count
                }
                KeyCode::Enter | KeyCode::Char(' ') => return Ok(selected as u16),
                KeyCode::Char(ch) => {
                    if counted {
                        selected = match ch {
                            'q' | 'Q' => return Err(MenuError::FullQuit),
                            '1'..='9' => ch as usize - '1' as usize,
                            _ => selected,
                        }
                        .clamp(0, opt_count - 1);
                    }
                }
                KeyCode::Esc => return Err(MenuError::Exit),
                _ => {}
            },
            _ => {}
        }

        renderer.on_event(&event)?;

        menu_drawable.selected = selected;
        menu_drawable.draw(pos, frame);
        renderer.render()?;
    }
}

pub fn choice_menu<'a, T>(
    renderer: &mut Renderer,
    frame: &mut impl CanvasLike,
    box_style: ContentStyle,
    text_style: ContentStyle,
    title: &str,
    options: &'a [(T, &str)],
    default: Option<usize>,
    counted: bool,
) -> Result<&'a T, MenuError> {
    let _options: Vec<&str> = options.iter().map(|opt| opt.1).collect();
    Ok(&options[menu(
        renderer, frame, box_style, text_style, title, &_options, default, counted,
    )? as usize]
        .0)
}

pub struct Menu<'a> {
    pub title: &'a str,
    pub options: &'a [&'a str],
    pub counted: bool,
    pub selected: usize,
    pub default: Option<usize>,
    pub box_style: ContentStyle,
    pub text_style: ContentStyle,
}

impl Menu<'_> {
    pub fn size(&self) -> UiDims {
        match self.options.iter().map(|opt| opt.len()).max() {
            Some(l) => UiDims::new(
                ((2 + if self.counted {
                    self.max_i_len() + 2
                } else {
                    0
                } + l
                    - 2)
                .max(self.title.len() + 2)
                    + 2) as i32
                    + 2,
                self.options.len() as i32 + 2 + 2,
            ),
            None => UiDims::new(0, 0),
        }
    }

    fn max_i_len(&self) -> usize {
        (self.options.len() + 1).to_string().len()
    }
}

impl<'a> Drawable for Menu<'a> {
    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
        UiBox::new(self.size(), self.box_style).draw(pos, frame);

        let mut inner = Frame::new(frame)
            .with_size((self.size() - UiDims::new(2, 2)).into())
            .with_pos((Into::<UiDims>::into(pos) + UiDims::new(1, 1)).into());
        inner.clear();

        (self.title, self.text_style).draw((2, 0), &mut inner);
        ("─".repeat(inner.size().0 as usize - 2), self.box_style).draw((1, 1), &mut inner);

        let is_default_enabled = self.default.is_some();

        for (i, option) in self.options.iter().enumerate() {
            let is_selected = i == self.selected;
            let is_default = self.default.map_or(false, |d| d == i);

            let mut option_text = String::default();
            if is_selected {
                option_text.push_str("> ");
            } else {
                option_text.push_str("  ");
            }

            if self.counted {
                option_text.push_str(&format!("{}.", i + 1).pad_to_width(self.max_i_len() + 2));
            }

            if is_default_enabled {
                if is_default {
                    option_text.push_str("▶ ");
                } else {
                    option_text.push_str("  ");
                }
            }

            option_text.push_str(&option.pad_to_width(inner.size().0 as usize));

            let style = if is_selected {
                ContentStyle {
                    background_color: Some(
                        self.text_style.foreground_color.unwrap_or(Color::White),
                    ),
                    foreground_color: Some(
                        self.text_style.background_color.unwrap_or(Color::Black),
                    ),
                    ..Default::default()
                }
            } else {
                self.text_style
            };

            (option_text, style).draw((0, i as i32 + 2), &mut inner);
        }
    }
}