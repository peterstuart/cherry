use super::container::Axis;
use embedded_graphics::prelude::*;

pub trait AxisSize<T> {
    fn for_axis(&self, axis: Axis) -> T;
}

impl AxisSize<u32> for Size {
    fn for_axis(&self, axis: Axis) -> u32 {
        match axis {
            Axis::Horizontal => self.width,
            Axis::Vertical => self.height,
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
}
