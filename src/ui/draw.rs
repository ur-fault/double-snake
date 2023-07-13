use std::{cell::RefCell, ops::DerefMut};

use crossterm::style::ContentStyle;
pub use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    terminal::size,
};

pub use substring::Substring;
use terminal_renderer::renderer::{Dims, Renderer};

// pub fn draw_str<'a>(
//     mut renderer: impl DerefMut<Target = &'a mut Renderer>,
//     mut x: i32,
//     y: i32,
//     mut text: &str,
//     style: ContentStyle,
// ) {
//     if y < 0 {
//         return;
//     }

//     if x < 0 && text.len() as i32 > -x + 1 {
//         text = text.substring(-x as usize, text.len() - 1);
//         x = 0;
//     }

//     if x > u16::MAX as i32 || y > u16::MAX as i32 {
//         return;
//     }

//     // renderer.draw_str(x as u16, y as u16, text, style);
//     renderer.frame().draw((x as u16, y as u16), (text, style));
// }

// pub fn draw_char<'a>(
//     mut renderer: impl DerefMut<Target = &'a mut Renderer>,
//     x: i32,
//     y: i32,
//     text: char,
//     style: ContentStyle,
// ) {
//     if y < 0 || x < 0 || x > u16::MAX as i32 || y > u16::MAX as i32 {
//         return;
//     }

//     // renderer.draw_char(x as u16, y as u16, text, style);
//     renderer.frame().draw((x as u16, y as u16), (text, style));
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Frame {
//     pub start: Dims,
//     pub end: Dims,
// }

// impl Frame {
//     pub fn new(start: Dims, end: Dims) -> Self {
//         Self { start, end }
//     }

//     pub fn new_sized(start: Dims, size: Dims) -> Self {
//         Self::new(start, (start.0 + size.0, start.1 + size.1))
//     }

//     pub fn size(&self) -> Dims {
//         (self.end.0 - self.start.0, self.end.1 - self.start.1) + (1, 1)
//     }

//     pub fn contains(&self, pos: Dims) -> bool {
//         pos.0 >= self.start.0 && pos.0 <= self.end.0 && pos.1 >= self.start.1 && pos.1 <= self.end.1
//     }

//     pub fn trim_absolute<'a>(&'a self, text: &'a impl AsRef<str>, mut pos: Dims) -> (&str, Dims) {
//         let mut text = text.as_ref();
//         let size = self.size();

//         if pos.1 < self.start.1 || pos.1 > self.end.1 {
//             return ("", pos);
//         }

//         if pos.0 < self.start.0 {
//             let offset = self.start.0 - pos.0;
//             text = text.substring(offset as usize, text.chars().count());
//             pos = Dims(self.start.0, pos.1);
//         }

//         if text.chars().count() as i32 + pos.0 > self.end.0 {
//             let x = size.0 - (pos.0 - self.start.0);
//             let x = x.max(0) as usize;
//             text = text.substring(0, x);
//         }

//         (text, pos)
//     }

//     #[allow(dead_code)]
//     pub fn trim_relative<'a>(&'a self, text: &'a impl AsRef<str>, pos: Dims) -> (&str, Dims) {
//         let (text, pos) = self.trim_absolute(text, pos + self.start);
//         (text, pos - self.start)
//     }

//     pub fn with_margin(&self, margin: Dims) -> Self {
//         Self {
//             start: self.start + margin,
//             end: self.end - margin,
//         }
//     }
// }
