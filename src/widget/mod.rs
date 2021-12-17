pub mod container;
pub mod text;

use embedded_graphics::prelude::*;

pub trait Widget<Color> {
    fn draw<Display, Error>(
        &self,
        display: &mut Display,
        origin: Point,
        size: Size,
    ) -> Result<(), Error>
    where
        Display: DrawTarget<Color = Color, Error = Error>;
}
