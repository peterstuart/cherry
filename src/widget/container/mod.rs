mod alignment;
mod axis;
mod border;
mod insets;
mod justification;
mod options;

pub use alignment::Alignment;
pub use axis::Axis;
pub use border::Border;
pub use insets::{Inset, Insets};
pub use justification::Justification;
pub use options::Options;

use super::{axis_size::AxisSize, IntrinsicSize, Widget};
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle, RoundedRectangle},
};

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

    fn main_axis(&self) -> Axis {
        self.options.axis
    }

    fn cross_axis(&self) -> Axis {
        self.options.axis.opposite()
    }

    fn content_size(&self) -> IntrinsicSize {
        self.options
            .children
            .iter()
            .fold(IntrinsicSize::none(), |size, widget| {
                let widget_size = widget.intrinsic_size();

                let cross_axis_dimension = match (
                    size.for_axis(self.cross_axis()),
                    widget_size.for_axis(self.cross_axis()),
                ) {
                    (Some(size), Some(widget_size)) => Some(size.max(widget_size)),
                    (Some(size), None) => Some(size),
                    (None, Some(widget_size)) => Some(widget_size),
                    (None, None) => None,
                };

                let main_axis_dimension = match (
                    size.for_axis(self.main_axis()),
                    widget_size.for_axis(self.main_axis()),
                ) {
                    (Some(size), Some(widget_size)) => Some(size + widget_size),
                    (Some(size), None) => Some(size),
                    (None, Some(widget_size)) => Some(widget_size),
                    (None, None) => None,
                };

                match self.main_axis() {
                    Axis::Horizontal => {
                        IntrinsicSize::new(main_axis_dimension, cross_axis_dimension)
                    }
                    Axis::Vertical => IntrinsicSize::new(cross_axis_dimension, main_axis_dimension),
                }
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

        let total_children_main_axis_dimension =
            self.content_size().for_axis(self.main_axis()).unwrap_or(0);
        let unused_main_axis_dimension =
            size.for_axis(self.main_axis()) - total_children_main_axis_dimension;

        let (mut current_main_axis_pos, space) = match self.options.justification {
            Justification::Start => (0, 0),
            Justification::Center => (unused_main_axis_dimension / 2, 0),
            Justification::End => (unused_main_axis_dimension, 0),
            Justification::SpaceBetween => {
                let space = if num_children > 1 {
                    unused_main_axis_dimension / (num_children - 1)
                } else {
                    0
                };
                (0, space)
            }
            Justification::SpaceAround => {
                let space = unused_main_axis_dimension / num_children;
                (space / 2, space)
            }
            Justification::SpaceEvenly => {
                let space = unused_main_axis_dimension / (num_children + 1);
                (space, space)
            }
        };

        for child in &self.options.children {
            let child_size = child.intrinsic_size().to_size_with_defaults(Size::zero());

            let cross_axis_offset = match self.options.alignment {
                Alignment::Start => 0,
                Alignment::Center => {
                    (size.for_axis(self.cross_axis()) - child_size.for_axis(self.cross_axis())) / 2
                }
                Alignment::End => {
                    size.for_axis(self.cross_axis()) - child_size.for_axis(self.cross_axis())
                }
            };

            let child_origin = match self.main_axis() {
                Axis::Horizontal => Point::new(
                    origin.x + current_main_axis_pos as i32,
                    origin.y + cross_axis_offset as i32,
                ),
                Axis::Vertical => Point::new(
                    origin.x + cross_axis_offset as i32,
                    origin.y + current_main_axis_pos as i32,
                ),
            };

            child.draw(display, child_origin, child_size)?;
            current_main_axis_pos += child_size.for_axis(self.main_axis()) + space;
        }

        Ok(())
    }
}

impl<Display> Widget<Display> for Container<Display>
where
    Display: DrawTarget,
{
    fn intrinsic_size(&self) -> IntrinsicSize {
        let total_size = self
            .content_size()
            .outset(self.options.padding)
            .outset(self.options.margin);

        IntrinsicSize::new(
            self.options.width.or(total_size.width),
            self.options.height.or(total_size.height),
        )
    }

    fn draw(&self, display: &mut Display, origin: Point, size: Size) -> Result<(), Display::Error> {
        let box_origin = Point::new(
            origin.x + self.options.margin.left as i32,
            origin.y + self.options.margin.top as i32,
        );
        let box_size = size.inset(self.options.margin);
        self.draw_self(display, box_origin, box_size)?;

        let content_origin = Point::new(
            box_origin.x + self.options.padding.left as i32,
            box_origin.y + self.options.padding.top as i32,
        );
        let content_size = box_size.inset(self.options.padding);
        self.draw_children(display, content_origin, content_size)
    }
}
