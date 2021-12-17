use super::Widget;
use embedded_graphics::{
    prelude::*,
    primitives::{CornerRadii, PrimitiveStyleBuilder, Rectangle, RoundedRectangle},
};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Border<Color> {
    pub color: Color,
    pub width: u32,
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Options<Color> {
    pub background_color: Option<Color>,
    pub border: Option<Border<Color>>,
    pub corner_radii: Option<CornerRadii>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Container<Color> {
    options: Options<Color>,
}

impl<Color> Container<Color>
where
    Color: PixelColor,
{
    pub fn new(options: Options<Color>) -> Self {
        Self { options }
    }
}

impl<Color> Widget<Color> for Container<Color>
where
    Color: PixelColor,
{
    fn draw<Display, Error>(
        &self,
        display: &mut Display,
        origin: Point,
        size: Size,
    ) -> Result<(), Error>
    where
        Display: DrawTarget<Color = Color, Error = Error>,
    {
        let mut style = PrimitiveStyleBuilder::new();

        if let Some(background_color) = self.options.background_color {
            style = style.fill_color(background_color);
        }

        if let Some(border) = self.options.border {
            style = style.stroke_color(border.color).stroke_width(border.width);
        }

        let style = style.build();
        let rectangle = Rectangle::new(origin, size);

        match self.options.corner_radii {
            Some(corner_radii) => RoundedRectangle::new(rectangle, corner_radii)
                .into_styled(style)
                .draw(display),
            None => rectangle.into_styled(style).draw(display),
        }
    }
}
