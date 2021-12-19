use super::Widget;
use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    text::{self, Baseline},
};

#[derive(Clone, Copy)]
pub struct Options<Color> {
    pub character_style: MonoTextStyle<'static, Color>,
}

#[derive(Clone, Copy)]
pub struct Text<Color> {
    options: Options<Color>,
    text: &'static str,
}

impl<Color> Text<Color>
where
    Color: PixelColor,
{
    pub fn new(options: Options<Color>, text: &'static str) -> Self {
        Self { options, text }
    }

    fn text(&self, origin: Point) -> text::Text<MonoTextStyle<Color>> {
        text::Text::with_baseline(
            self.text,
            origin,
            self.options.character_style,
            Baseline::Top,
        )
    }
}

impl<Color, Display> Widget<Display> for Text<Color>
where
    Color: PixelColor,
    Display: DrawTarget<Color = Color>,
{
    fn intrinsic_size(&self) -> Size {
        self.text(Point::zero()).bounding_box().size
    }

    fn draw(
        &self,
        display: &mut Display,
        origin: Point,
        _: Option<Size>,
    ) -> Result<(), Display::Error> {
        let text = self.text(origin);
        text.draw(display)?;

        Ok(())
    }
}
