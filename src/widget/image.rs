use super::{IntrinsicSize, Widget};
use embedded_graphics::{image, prelude::*, primitives::Rectangle};

#[derive(Clone, Copy)]
pub struct Image<'a, T>
where
    T: ImageDrawable,
{
    image: &'a T,
}

impl<'a, T> Image<'a, T>
where
    T: ImageDrawable,
{
    pub fn new(image: &'a T) -> Self {
        Self { image }
    }
}

impl<'a, T, Display> Widget<Display> for Image<'a, T>
where
    T: 'a + ImageDrawable,
    Display: DrawTarget<Color = T::Color>,
{
    fn intrinsic_size(&self) -> IntrinsicSize {
        self.image.size().into()
    }

    fn draw(&self, display: &mut Display, origin: Point, size: Size) -> Result<(), Display::Error> {
        image::Image::new(self.image, origin)
            .draw(&mut display.clipped(&Rectangle::new(origin, size)))
    }
}
