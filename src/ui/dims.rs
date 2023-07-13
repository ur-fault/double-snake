use std::ops::{Add, Mul, Sub};

use terminal_renderer::renderer::Dims;

#[derive(Copy, Clone)]
pub struct UiDims {
    pub x: i32,
    pub y: i32,
}

impl UiDims {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<Dims> for UiDims {
    fn from(dims: Dims) -> Self {
        Self {
            x: dims.0,
            y: dims.1,
        }
    }
}

impl From<UiDims> for Dims {
    fn from(dims: UiDims) -> Self {
        (dims.x, dims.y)
    }
}

impl Add for UiDims {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for UiDims {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for UiDims {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

pub trait IntoUidims {
    fn into_ui(self) -> UiDims;
}

impl<D> IntoUidims for D
where
    D: Into<UiDims>,
{
    fn into_ui(self) -> UiDims {
        self.into()
    }
}
