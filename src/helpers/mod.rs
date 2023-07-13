use crossterm::{event::KeyEventKind, style::ContentStyle};

pub fn is_key_release(kind: KeyEventKind) -> bool {
    match kind {
        KeyEventKind::Release => true,
        _ => false,
    }
}

pub fn value_if<T: Default>(cond: bool, fun: impl FnOnce() -> T) -> T {
    if cond {
        fun()
    } else {
        T::default()
    }
}

pub fn value_if_else<T>(cond: bool, fun: impl FnOnce() -> T, else_fun: impl FnOnce() -> T) -> T {
    if cond {
        fun()
    } else {
        else_fun()
    }
}

pub fn swap_style(style: ContentStyle) -> ContentStyle {
    let mut new_style = ContentStyle::new();
    new_style.background_color = style.foreground_color;
    new_style.foreground_color = style.background_color;
    new_style
}