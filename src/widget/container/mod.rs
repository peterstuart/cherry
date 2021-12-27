mod alignment;
mod axis;
mod border;
mod insets;
mod justification;

pub use alignment::Alignment;
pub use axis::Axis;
pub use border::Border;
pub use insets::{Inset, Insets};
pub use justification::Justification;

use super::{axis_size::AxisSize, IntrinsicSize, LayoutOptions, Widget};
use alloc::{boxed::Box, vec::Vec};
use cherry_macros::Builder;
use embedded_graphics::{
    prelude::*,
    primitives::{CornerRadii, PrimitiveStyleBuilder, Rectangle, RoundedRectangle},
};

#[derive(Builder)]
pub struct Container<Display>
where
    Display: DrawTarget,
{
    alignment: Alignment,
    axis: Axis,
    background_color: Option<Display::Color>,
    border: Option<Border<Display::Color>>,
    children: Vec<Box<dyn Widget<Display>>>,
    corner_radii: Option<CornerRadii>,
    height: Option<u32>,
    justification: Justification,
    layout_options: LayoutOptions,
    margin: Insets,
    padding: Insets,
    width: Option<u32>,
}

impl<Display> Default for Container<Display>
where
    Display: DrawTarget,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Display> Container<Display>
where
    Display: DrawTarget,
{
    pub fn new() -> Self {
        Self {
            alignment: Default::default(),
            axis: Default::default(),
            background_color: Default::default(),
            border: Default::default(),
            children: Default::default(),
            corner_radii: Default::default(),
            height: Default::default(),
            justification: Default::default(),
            layout_options: Default::default(),
            margin: Default::default(),
            padding: Default::default(),
            width: Default::default(),
        }
    }

    fn main_axis(&self) -> Axis {
        self.axis
    }

    fn cross_axis(&self) -> Axis {
        self.axis.opposite()
    }

    fn content_size(&self) -> IntrinsicSize {
        self.children
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

    fn border_width(&self) -> u32 {
        self.border.map_or(0, |border| border.width)
    }

    fn draw_self(
        &self,
        display: &mut Display,
        origin: Point,
        size: Size,
    ) -> Result<(), Display::Error> {
        let mut style = PrimitiveStyleBuilder::new();

        if let Some(background_color) = self.background_color {
            style = style.fill_color(background_color);
        }

        if let Some(border) = self.border {
            style = style.stroke_color(border.color).stroke_width(border.width);
        }

        let style = style.build();
        let rectangle = Rectangle::new(origin, size);

        match self.corner_radii {
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
        let num_children = self.children.len() as u32;

        if num_children == 0 {
            return Ok(());
        }

        let total_children_main_axis_dimension = self
            .content_size()
            .for_axis(self.main_axis())
            .unwrap_or(0)
            .min(size.for_axis(self.main_axis()));
        let extra_main_axis_dimension =
            size.for_axis(self.main_axis()) - total_children_main_axis_dimension;
        let grow_total: u32 = self
            .children
            .iter()
            .map(|child| child.layout_options().grow)
            .sum();

        let (unused_main_axis_dimension, grow_unit) = if grow_total > 0 {
            (0, extra_main_axis_dimension / grow_total)
        } else {
            (extra_main_axis_dimension, 0)
        };

        let (mut current_main_axis_pos, space) = match self.justification {
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

        for child in &self.children {
            let child_alignment = child.layout_options().alignment.unwrap_or(self.alignment);

            let default_size = match (child_alignment, self.main_axis()) {
                (Alignment::Stretch, Axis::Horizontal) => Size::new(0, size.height),
                (Alignment::Stretch, Axis::Vertical) => Size::new(size.width, 0),
                _ => Size::zero(),
            };

            let mut child_size = child
                .intrinsic_size()
                .to_size_with_defaults(default_size)
                .component_min(size);
            child_size.add_to_axis(grow_unit * child.layout_options().grow, self.main_axis());

            let cross_axis_offset = match child_alignment {
                Alignment::Stretch | Alignment::Start => 0,
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
            .outset(self.padding)
            .outset(Insets::all(self.border_width()))
            .outset(self.margin);

        IntrinsicSize::new(
            self.width.or(total_size.width),
            self.height.or(total_size.height),
        )
    }

    fn layout_options(&self) -> LayoutOptions {
        self.layout_options
    }

    fn draw(&self, display: &mut Display, origin: Point, size: Size) -> Result<(), Display::Error> {
        let box_origin = Point::new(
            origin.x + self.margin.left as i32,
            origin.y + self.margin.top as i32,
        );

        // outer half of the border
        let outer_border = Insets::all(self.border_width() / 2);

        let box_size = size.inset(self.margin).inset(outer_border);
        self.draw_self(display, box_origin, box_size)?;

        // inner half of the border (the inner half gets the remainder
        // when the border width isn't divisble by 2)
        let inner_border_width = self.border_width() / 2 + self.border_width() % 2;
        let inner_border = Insets::all(inner_border_width);

        let content_origin = Point::new(
            box_origin.x + inner_border_width as i32 + self.padding.left as i32,
            box_origin.y + inner_border_width as i32 + self.padding.top as i32,
        );
        let content_size = box_size.inset(inner_border).inset(self.padding);
        self.draw_children(display, content_origin, content_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb888};

    #[test]
    fn child_bigger_than_self() {
        let mut display: MockDisplay<Rgb888> = MockDisplay::new();
        let size = display.size();

        let child = Container::new()
            .width(Some(size.width + 10))
            .height(Some(size.height + 10));
        let container = Container::new().children(vec![child.boxed()]);

        container.draw(&mut display, Point::zero(), size).unwrap()
    }
}
