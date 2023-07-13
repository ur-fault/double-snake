use crossterm::style::ContentStyle;
use terminal_renderer::{canvas::CanvasLike, drawable::Drawable, renderer::Dims};

use super::dims::UiDims;

pub mod helpers {
    use crate::ui::dims::UiDims;

    pub fn box_center(container_size: UiDims, box_size: UiDims) -> UiDims {
        let x = (container_size.x - box_size.x) / 2;
        let y = (container_size.y - box_size.y) / 2;
        UiDims::new(x, y)
    }
}

pub struct UiBox {
    pub size: UiDims,
    pub style: ContentStyle,
}

impl UiBox {
    pub fn new(size: UiDims, style: ContentStyle) -> Self {
        Self { size, style }
    }
}

impl Drawable for UiBox {
    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
        (
            format!("╭{}╮", "─".repeat(self.size.x as usize - 2)),
            self.style,
        )
            .draw((pos.0, pos.1), frame);

        for y in pos.1 + 1..pos.1 + self.size.y - 1 {
            ('│', self.style).draw((pos.0, y), frame);
            ('│', self.style).draw((pos.0 + self.size.x - 1, y), frame);
        }

        (
            format!("╰{}╯", "─".repeat(self.size.x as usize - 2)),
            self.style,
        )
            .draw((pos.0, pos.1 + self.size.y - 1), frame);
    }
}
