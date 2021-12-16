use super::Widget;
use alloc::{boxed::Box, vec::Vec};
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

pub struct Container<Display>
where
    Display: DrawTarget,
{
    options: Options<Display::Color>,
    children: Vec<Box<dyn Widget<Display>>>,
}

impl<Display> Container<Display>
where
    Display: DrawTarget,
{
    pub fn new(options: Options<Display::Color>, children: Vec<Box<dyn Widget<Display>>>) -> Self {
        Self { options, children }
    }

    fn draw_self(
        &self,
        display: &mut Display,
        origin: Point,
        size: Size,
    ) -> Result<(), Display::Error> {
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

    fn draw_children(
        &self,
        display: &mut Display,
        origin: Point,
        max_size: Size,
    ) -> Result<Size, Display::Error> {
        let mut total_size = Size::new(max_size.width, 0);

        for child in &self.children {
            let current_origin = Point::new(origin.x, origin.y + (total_size.height as i32));
            let remaining_size = Size::new(max_size.width, max_size.height - total_size.height);
            let consumed_size = child.draw(display, current_origin, remaining_size)?;

            total_size.width = total_size.width.max(consumed_size.width);
            total_size.height += consumed_size.height;
        }

        Ok(total_size)
    }
}

impl<Display> Widget<Display> for Container<Display>
where
    Display: DrawTarget,
{
    fn draw(
        &self,
        display: &mut Display,
        origin: Point,
        max_size: Size,
    ) -> Result<Size, Display::Error> {
        self.draw_self(display, origin, max_size)?;
        self.draw_children(display, origin, max_size)
    }
}
