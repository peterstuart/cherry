use super::{IntrinsicSize, LayoutOptions, Widget};
use cherry_macros::Builder;
use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    primitives::Rectangle,
    text::{self, Baseline},
};

#[derive(Clone, Copy, Builder)]
pub struct Text<'font, 'text, Color> {
    #[omit]
    character_style: MonoTextStyle<'font, Color>,
    layout_options: LayoutOptions,
    #[omit]
    text: &'text str,
}

impl<'font, 'text, Color> Text<'font, 'text, Color>
where
    Color: PixelColor,
{
    pub fn new(text: &'text str, character_style: MonoTextStyle<'font, Color>) -> Self {
        Self {
            character_style,
            layout_options: Default::default(),
            text,
        }
    }

    fn text(&self, origin: Point) -> text::Text<MonoTextStyle<Color>> {
        text::Text::with_baseline(self.text, origin, self.character_style, Baseline::Top)
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

    fn layout_options(&self) -> LayoutOptions {
        self.layout_options
    }

    fn draw(&self, display: &mut Display, origin: Point, size: Size) -> Result<(), Display::Error> {
        let text = self.text(origin);
        let mut display = display.clipped(&Rectangle::new(origin, size));
        text.draw(&mut display)?;

        Ok(())
    }
}
