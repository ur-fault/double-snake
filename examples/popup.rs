use crossterm::style::{ContentStyle, Stylize};
use double_snake::ui::{popup, CrosstermError};
use terminal_renderer::renderer::Renderer;

fn main() -> Result<(), CrosstermError> {
    let mut renderer = Renderer::new()?;

    let mut render_space = renderer.get_render_space();

    popup::popup(
        &mut renderer,
        &mut render_space,
        ContentStyle::new().red(),
        ContentStyle::default(),
        "Title",
        &["Line 1", "Line 2", "Line 3", "Much Much Longer Line 4"],
        // &[],
    )?;

    Ok(())
}
