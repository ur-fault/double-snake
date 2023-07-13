use crossterm::style::{ContentStyle, Stylize};
use double_snake::ui::menu::{self, MenuError};
use terminal_renderer::renderer::Renderer;

fn main() -> Result<(), MenuError> {
    let mut renderer = Renderer::new()?;

    let mut render_space = renderer.get_render_space();

    let i = menu::menu(
        &mut renderer,
        &mut render_space,
        ContentStyle::new().cyan(),
        ContentStyle::default(),
        "Title",
        &["Line 1", "Line 2", "Line 3", "Much Much Longer Line 4"],
        None,
        true,
    )?;

    drop(renderer);

    println!("You chose: {}", i + 1);

    Ok(())
}
