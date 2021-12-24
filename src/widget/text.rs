use super::{IntrinsicSize, Widget};
use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    primitives::Rectangle,
    text::{self, Baseline},
};

#[derive(Clone, Copy)]
pub struct Options<'font, Color> {
    pub character_style: MonoTextStyle<'font, Color>,
}

#[derive(Clone, Copy)]
pub struct Text<'font, 'text, Color> {
    options: Options<'font, Color>,
    text: &'text str,
}

impl<'font, 'text, Color> Text<'font, 'text, Color>
where
    Color: PixelColor,
{
    pub fn new(options: Options<'font, Color>, text: &'text str) -> Self {
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

impl<'font, 'text, Color, Display> Widget<Display> for Text<'font, 'text, Color>
where
    Color: PixelColor,
    Display: DrawTarget<Color = Color>,
{
    fn intrinsic_size(&self) -> IntrinsicSize {
        self.text(Point::zero()).bounding_box().size.into()
    }

    fn draw(&self, display: &mut Display, origin: Point, size: Size) -> Result<(), Display::Error> {
        let text = self.text(origin);
        let mut display = display.clipped(&Rectangle::new(origin, size));
        text.draw(&mut display)?;

        Ok(())
    }
}
