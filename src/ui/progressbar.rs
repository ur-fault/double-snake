use crossterm::style::ContentStyle;
pub use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    terminal::size,
};
use tap::Tap;
use terminal_renderer::{
    canvas::CanvasLike,
    drawable::Drawable,
    frame::Frame,
    renderer::{Dims, Renderer},
};

use super::{dims::IntoUidims, uibox::UiBox, *};

pub fn show_fullwidth_progressbar(
    renderer: &mut Renderer,
    frame: &mut impl CanvasLike,
    box_style: ContentStyle,
    text_style: ContentStyle,
    title: &str,
    progress: f64,
) -> Result<(), CrosstermError> {
    let progressbar = ProgressBar {
        title,
        progress,
        box_style,
        text_style,
        min_width: Some(frame.size().0),
    };
    let pos = uibox::helpers::box_center(
        frame.size().into_ui(),
        frame
            .size()
            .into_ui()
            .tap_mut(|s| s.y = progressbar.size().y),
    );
    progressbar.draw(pos.into(), frame);
    renderer.render()?;

    Ok(())
}

pub fn show_progressbar(
    renderer: &mut Renderer,
    frame: &mut impl CanvasLike,
    box_style: ContentStyle,
    text_style: ContentStyle,
    title: &str,
    progress: f64,
    min_width: Option<i32>,
) -> Result<(), CrosstermError> {
    let progressbar = ProgressBar {
        title,
        progress,
        box_style,
        text_style,
        min_width,
    };
    let pos = uibox::helpers::box_center(frame.size().into(), progressbar.size().into());
    progressbar.draw(pos.into(), frame);
    renderer.render()?;

    Ok(())
}

pub struct ProgressBar<'a> {
    pub title: &'a str,
    pub progress: f64,
    pub box_style: ContentStyle,
    pub text_style: ContentStyle,
    pub min_width: Option<i32>,
}

impl<'a> ProgressBar<'a> {
    pub fn size(&self) -> UiDims {
        UiDims::new(
            (self.title.len() as i32 + 4).max(self.min_width.unwrap_or(0)),
            4,
        )
    }
}

impl<'a> Drawable for ProgressBar<'a> {
    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
        let real_size = self.size();
        UiBox::new(real_size, self.box_style).draw(pos, frame);
        let mut inner = Frame::new(frame).centered((real_size - UiDims::new(2, 2)).into());
        (self.title, self.text_style).draw((1, 0), &mut inner);
        (
            "█".repeat(((real_size.x as usize - 4) as f64 * self.progress) as usize),
            self.box_style,
        )
            .draw((1, 1), &mut inner);
    }
}
