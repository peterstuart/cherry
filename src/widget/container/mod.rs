mod alignment;
mod border;
mod sizing;

pub use alignment::Alignment;
pub use border::Border;
pub use sizing::Sizing;

use super::Widget;
use alloc::{boxed::Box, vec::Vec};
use embedded_graphics::{
    prelude::*,
    primitives::{CornerRadii, PrimitiveStyleBuilder, Rectangle, RoundedRectangle},
};

pub struct Options<Display>
where
    Display: DrawTarget,
{
    pub alignment: Alignment,
    pub background_color: Option<Display::Color>,
    pub border: Option<Border<Display::Color>>,
    pub corner_radii: Option<CornerRadii>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub children: Vec<Box<dyn Widget<Display>>>,
}

impl<Display> Default for Options<Display>
where
    Display: DrawTarget,
{
    fn default() -> Self {
        Self {
            alignment: Default::default(),
            background_color: Default::default(),
            border: Default::default(),
            corner_radii: Default::default(),
            width: Default::default(),
            height: Default::default(),
            children: Default::default(),
        }
    }
}

pub struct Container<Display>
where
    Display: DrawTarget,
{
    options: Options<Display>,
}

impl<Display> Container<Display>
where
    Display: DrawTarget,
{
    pub fn new(options: Options<Display>) -> Self {
        Self { options }
    }

    fn content_size(&self) -> Size {
        self.options
            .children
            .iter()
            .fold(Size::zero(), |size, child| {
                let child_size = child.intrinsic_size();
                Size::new(
                    size.width.max(child_size.width),
                    size.height + child_size.height,
                )
            })
    }

    fn draw_self(
        &self,
        display: &mut Display,
        origin: Point,
        size: Option<Size>,
    ) -> Result<(), Display::Error> {
        let size = size.unwrap_or_else(|| self.intrinsic_size());

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
        _: Option<Size>,
    ) -> Result<(), Display::Error> {
        let mut y: i32 = 0;

        for child in &self.options.children {
            let child_origin = Point::new(origin.x, origin.y + y);
            child.draw(display, child_origin, None)?;
            y += child.intrinsic_size().height as i32;
        }

        Ok(())
    }
}

impl<Display> Widget<Display> for Container<Display>
where
    Display: DrawTarget,
{
    fn intrinsic_size(&self) -> Size {
        let content_size = self.content_size();

        Size::new(
            self.options.width.unwrap_or(content_size.width),
            self.options.height.unwrap_or(content_size.height),
        )
    }

    fn draw(
        &self,
        display: &mut Display,
        origin: Point,
        size: Option<Size>,
    ) -> Result<(), Display::Error> {
        self.draw_self(display, origin, size)?;
        self.draw_children(display, origin, size)
    }
}
