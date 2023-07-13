use terminal_renderer::canvas::CanvasLike;

pub struct DrawContext<'a> {
    pub renderer: CanvasLike,
    pub style: ContentStyle,
    pub frame: Option<Frame>,
}

#[allow(dead_code)]
impl<'a> DrawContext<'a> {
    pub fn draw_char(&mut self, pos: Dims, text: char) {
        self.draw_char_styled(pos, text, self.style);
    }

    pub fn draw_str(&mut self, pos: Dims, text: &str) {
        self.draw_str_styled(pos, text, self.style);
    }

    pub fn draw_box(&mut self, pos: Dims, size: Dims) {
        draw_box(self.renderer.borrow_mut(), pos, size, self.style);
    }

    pub fn draw_char_styled(&mut self, pos: Dims, text: char, style: ContentStyle) {
        if self.frame.as_ref().map_or(true, |f| f.contains(pos)) {
            draw_char(self.renderer.borrow_mut(), pos.0, pos.1, text, style);
        }
    }

    pub fn draw_str_styled(&mut self, pos: Dims, text: &str, style: ContentStyle) {
        let (text, pos) = self
            .frame
            .as_ref()
            .map_or((text, pos), |f| f.trim_absolute(&text, pos));
        draw_str(self.renderer.borrow_mut(), pos.0, pos.1, text, style);
    }

    pub fn draw_box_styled(&mut self, pos: Dims, size: Dims, style: ContentStyle) {
        draw_box(self.renderer.borrow_mut(), pos, size, style);
    }
}

#[cfg(test)]
mod tests {
    use super::{Dims, Frame};

    #[test]
    fn frame_trim_absolute() {
        let frame = Frame::new_sized(Dims(0, 0), Dims(3, 1));
        let (text, ..) = frame.trim_absolute(&"123456", Dims(0, 0));
        assert_eq!(text, "123");

        let (text, ..) = frame.trim_absolute(&"123456", Dims(1, 0));
        assert_eq!(text, "12");

        let (text, ..) = frame.trim_absolute(&"123456", Dims(-1, 0));
        assert_eq!(text, "234");

        let (text, ..) = frame.trim_absolute(&"123456", Dims(-4, 0));
        assert_eq!(text, "56");

        let (text, ..) = frame.trim_absolute(&"123456", Dims(-3, 0));
        assert_eq!(text, "456");
    }
}
