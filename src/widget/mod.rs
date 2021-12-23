pub mod container;
pub mod image;
pub mod text;

mod axis_size;
mod intrinsic_size;
mod layout_options;

pub use intrinsic_size::IntrinsicSize;
pub use layout_options::LayoutOptions;

use alloc::boxed::Box;
use embedded_graphics::prelude::*;

pub trait Widget<Display: DrawTarget> {
    fn intrinsic_size(&self) -> IntrinsicSize;

    fn layout_options(&self) -> LayoutOptions {
        LayoutOptions::default()
    }

    fn draw(&self, display: &mut Display, origin: Point, size: Size) -> Result<(), Display::Error>;

    fn boxed(self) -> Box<dyn Widget<Display>>
    where
        Self: 'static + Sized,
    {
        Box::new(self)
    }
}
