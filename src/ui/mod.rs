pub use std::time::Duration;

use crossterm::{
    event::{Event, KeyCode, KeyEvent},
    Result as CResult,
};
pub use substring::Substring;
use terminal_renderer::helpers::term_size;

use crate::helpers;

use self::dims::UiDims;

pub mod dims;
pub mod draw;
pub mod menu;
pub mod popup;
pub mod progressbar;
pub mod uibox;

#[derive(Debug)]
pub struct CrosstermError(pub crossterm::ErrorKind);

impl From<crossterm::ErrorKind> for CrosstermError {
    fn from(error: crossterm::ErrorKind) -> Self {
        Self(error)
    }
}

pub fn box_center_screen(box_dims: UiDims) -> Result<UiDims, CrosstermError> {
    let size_u16 = term_size();
    Ok(uibox::helpers::box_center(
        UiDims::new(size_u16.0 as i32, size_u16.1 as i32),
        box_dims,
    ))
}

pub fn format_duration(dur: Duration) -> String {
    format!(
        "{}m{:.1}s",
        dur.as_secs() / 60,
        (dur.as_secs() % 60) as f32 + dur.subsec_millis() as f32 / 1000f32,
    )
}

pub fn format_days_duration(dur: Duration) -> String {
    format!("{} days", dur.as_secs() / 86400)
}

pub fn wait_for_key() -> CResult<KeyCode> {
    let mut e = crossterm::event::read();
    loop {
        match e {
            Ok(event) => match event {
                Event::Key(KeyEvent { code, kind, .. }) if !helpers::is_key_release(kind) => {
                    return Ok(code)
                }
                _ => e = crossterm::event::read(),
            },
            Err(e) => return Err(e),
        }
    }
}
