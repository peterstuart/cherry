pub mod container;
pub mod text;

mod axis_size;
mod intrinsic_size;

pub use intrinsic_size::IntrinsicSize;

use alloc::boxed::Box;
use embedded_graphics::prelude::*;

pub trait Widget<Display: DrawTarget> {
    fn intrinsic_size(&self) -> IntrinsicSize;

    fn grow(&self) -> u32 {
        0
    }

    fn draw(&self, display: &mut Display, origin: Point, size: Size) -> Result<(), Display::Error>;

    fn boxed(self) -> Box<dyn Widget<Display>>
    where
        Self: 'static + Sized,
    {
        Box::new(self)
    }
}
