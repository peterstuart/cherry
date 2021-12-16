use super::Widget;
use embedded_graphics::{mono_font::MonoTextStyle, prelude::*, text};

#[derive(Clone, Copy)]
pub struct Options<Color> {
    pub character_style: MonoTextStyle<'static, Color>,
}

#[derive(Clone, Copy)]
pub struct Text<Color> {
    options: Options<Color>,
}

impl<Color> Text<Color>
where
    Color: PixelColor,
{
    pub fn new(options: Options<Color>) -> Self {
        Self { options }
    }
}

impl<Color> Widget<Color> for Text<Color>
where
    Color: PixelColor,
{
    fn draw<Display, Error>(
        &self,
        display: &mut Display,
        origin: Point,
        _: Size,
    ) -> Result<(), Error>
    where
        Display: DrawTarget<Color = Color, Error = Error>,
    {
        text::Text::new("hello", origin, self.options.character_style).draw(display)?;

        Ok(())
    }
}
