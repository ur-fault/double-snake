use crossterm::{event::KeyEventKind, style::ContentStyle};
pub use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    terminal::size,
};
use terminal_renderer::{
    canvas::CanvasLike,
    drawable::{misc::CenteredStringExt, Drawable},
    frame::Frame,
    renderer::{Dims, Renderer},
};

use super::{
    dims::UiDims,
    uibox::{self, UiBox},
    CrosstermError,
};

pub fn popup(
    renderer: &mut Renderer,
    frame: &mut impl CanvasLike,
    box_style: ContentStyle,
    text_style: ContentStyle,
    title: &str,
    texts: &[&str],
) -> Result<KeyCode, CrosstermError> {
    let popup_drawable = Popup {
        title: title,
        texts: texts,
        box_style,
        text_style,
    };
    let pos = uibox::helpers::box_center(frame.size().into(), popup_drawable.size().into());
    popup_drawable.draw(pos.into(), frame);
    renderer.render()?;

    loop {
        let event = read()?;
        if let Event::Key(KeyEvent { code, kind, .. }) = event {
            if kind != KeyEventKind::Release {
                break Ok(code);
            }
        }

        renderer.on_event(&event)?;

        popup_drawable.draw(pos.into(), frame);
        renderer.render()?;
    }
}

pub struct Popup<'a> {
    pub title: &'a str,
    pub texts: &'a [&'a str],
    pub box_style: ContentStyle,
    pub text_style: ContentStyle,
}

impl<'a> Popup<'a> {
    fn size(&self) -> UiDims {
        match self.texts.iter().map(|text| text.len()).max() {
            Some(l) => UiDims::new(
                2 + 2 + l.max(self.title.len()) as i32,
                2 + 2 + self.texts.len() as i32,
            ),
            None => UiDims::new(4 + self.title.len() as i32, 3),
        }
    }
}

impl<'a> Drawable for Popup<'a> {
    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
        UiBox::new(self.size(), self.box_style).draw(pos, frame);

        let mut inner = Frame::new(frame)
            .with_size((self.size() - UiDims::new(2, 2)).into())
            .with_pos((Into::<UiDims>::into(pos) + UiDims::new(1, 1)).into());
        inner.clear();

        (self.title.to_owned().center(), self.text_style).draw((0, 0), &mut inner);
        if self.texts.is_empty() {
            return;
        }

        ("─".repeat(inner.size().0 as usize - 2), self.box_style).draw((1, 1), &mut inner);
        for (i, line) in self.texts.iter().enumerate() {
            (line.to_owned(), self.text_style).draw((1, i as i32 + 2), &mut inner);
        }
    }
}
