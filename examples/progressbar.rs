use std::{
    thread::{self},
    time::Duration,
};

use crossterm::style::{ContentStyle, Stylize};
use double_snake::ui::{
    progressbar::{show_fullwidth_progressbar, show_progressbar},
    CrosstermError,
};
use terminal_renderer::renderer::Renderer;

fn main() -> Result<(), CrosstermError> {
    let mut renderer = Renderer::new()?;

    let mut render_space = renderer.get_render_space();

    for i in 0..=10 {
        show_progressbar(
            &mut renderer,
            &mut render_space,
            ContentStyle::new().green(),
            ContentStyle::new(),
            "Fit Content",
            i as f64 / 10.0,
            None,
        )?;

        thread::sleep(Duration::from_millis(100));
    }

    thread::sleep(Duration::from_millis(300));
    for i in 0..=10 {
        show_progressbar(
            &mut renderer,
            &mut render_space,
            ContentStyle::new().green(),
            ContentStyle::new(),
            "Min Width 50",
            i as f64 / 10.0,
            Some(50),
        )?;

        thread::sleep(Duration::from_millis(100));
    }

    thread::sleep(Duration::from_millis(300));
    for i in 0..=10 {
        show_fullwidth_progressbar(
            &mut renderer,
            &mut render_space,
            ContentStyle::new().green(),
            ContentStyle::new(),
            "Full Width",
            i as f64 / 10.0,
        )?;

        thread::sleep(Duration::from_millis(100));
    }

    thread::sleep(Duration::from_millis(300));
    Ok(())
}
