use super::container::Axis;
use embedded_graphics::prelude::*;

pub trait AxisSize<T> {
    fn for_axis(&self, axis: Axis) -> T;

    fn add_to_axis(&mut self, value: u32, axis: Axis);
}

impl AxisSize<u32> for Size {
    fn for_axis(&self, axis: Axis) -> u32 {
        match axis {
            Axis::Horizontal => self.width,
            Axis::Vertical => self.height,
        }
    }

    fn add_to_axis(&mut self, value: u32, axis: Axis) {
        match axis {
            Axis::Horizontal => self.width += value,
            Axis::Vertical => self.height += value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_axis() {
        let size = Size::new(1, 2);
        assert_eq!(size.for_axis(Axis::Horizontal), size.width);
        assert_eq!(size.for_axis(Axis::Vertical), size.height);
    }

    #[test]
    fn add_to_axis() {
        let mut size = Size::new(1, 2);

        size.add_to_axis(10, Axis::Horizontal);
        assert_eq!(size, Size::new(11, 2));

        size.add_to_axis(10, Axis::Vertical);
        assert_eq!(size, Size::new(11, 12));
    }
}
