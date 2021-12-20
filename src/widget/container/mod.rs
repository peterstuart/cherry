mod alignment;
mod border;
mod justification;

pub use alignment::Alignment;
pub use border::Border;
pub use justification::Justification;

use super::{IntrinsicSize, Widget};
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
    pub children: Vec<Box<dyn Widget<Display>>>,
    pub corner_radii: Option<CornerRadii>,
    pub height: Option<u32>,
    pub justification: Justification,
    pub width: Option<u32>,
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
            children: Default::default(),
            corner_radii: Default::default(),
            height: Default::default(),
            justification: Default::default(),
            width: Default::default(),
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

    fn content_size(&self) -> IntrinsicSize {
        self.options
            .children
            .iter()
            .fold(IntrinsicSize::none(), |size, widget| {
                let widget_size = widget.intrinsic_size();

                let width = match (size.width, widget_size.width) {
                    (Some(width), Some(widget_width)) => Some(width.max(widget_width)),
                    (Some(width), None) => Some(width),
                    (None, Some(widget_width)) => Some(widget_width),
                    (None, None) => None,
                };

                let height = match (size.height, widget_size.height) {
                    (Some(height), Some(widget_height)) => Some(height + widget_height),
                    (Some(height), None) => Some(height),
                    (None, Some(widget_height)) => Some(widget_height),
                    (None, None) => None,
                };

                IntrinsicSize::new(width, height)
            })
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
        size: Size,
    ) -> Result<(), Display::Error> {
        let num_children = self.options.children.len() as u32;

        if num_children == 0 {
            return Ok(());
        }

        let total_children_height = self.content_size().height.unwrap_or(0);
        let unused_height = size.height - total_children_height;

        let (mut current_y, space) = match self.options.justification {
            Justification::Start => (0, 0),
            Justification::Center => (unused_height / 2, 0),
            Justification::End => (unused_height, 0),
            Justification::SpaceBetween => {
                let space = if num_children > 1 {
                    unused_height / (num_children - 1)
                } else {
                    0
                };
                (0, space)
            }
            Justification::SpaceAround => {
                let space = unused_height / num_children;
                (space / 2, space)
            }
            Justification::SpaceEvenly => {
                let space = unused_height / (num_children + 1);
                (space, space)
            }
        };

        for child in &self.options.children {
            let child_size = child
                .intrinsic_size()
                .to_size_with_defaults(Size::new(size.width, 0));

            let offset = match self.options.alignment {
                Alignment::Start => 0,
                Alignment::Center => (size.width - child_size.width) / 2,
                Alignment::End => size.width - child_size.width,
            };

            let child_origin = Point::new(origin.x + (offset as i32), origin.y + current_y as i32);
            child.draw(display, child_origin, child_size)?;
            current_y += child_size.height + space;
        }

        Ok(())
    }
}

impl<Display> Widget<Display> for Container<Display>
where
    Display: DrawTarget,
{
    fn intrinsic_size(&self) -> IntrinsicSize {
        let content_size = self.content_size();

        IntrinsicSize::new(
            self.options.width.or(content_size.width),
            self.options.height.or(content_size.height),
        )
    }

    fn draw(&self, display: &mut Display, origin: Point, size: Size) -> Result<(), Display::Error> {
        self.draw_self(display, origin, size)?;
        self.draw_children(display, origin, size)
    }
}
