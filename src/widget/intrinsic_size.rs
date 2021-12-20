use super::{
    axis_size::AxisSize,
    container::{Axis, Inset, Insets},
};
use embedded_graphics::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct IntrinsicSize {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl IntrinsicSize {
    pub fn new(width: Option<u32>, height: Option<u32>) -> Self {
        Self { width, height }
    }

    pub fn none() -> Self {
        Self {
            width: None,
            height: None,
        }
    }

    pub fn to_size(&self) -> Option<Size> {
        match (self.width, self.height) {
            (Some(width), Some(height)) => Some(Size::new(width, height)),
            _ => None,
        }
    }

    pub fn to_size_with_defaults(&self, default: Size) -> Size {
        Size::new(
            self.width.unwrap_or(default.width),
            self.height.unwrap_or(default.height),
        )
    }
}

impl AxisSize<Option<u32>> for IntrinsicSize {
    fn for_axis(&self, axis: Axis) -> Option<u32> {
        match axis {
            Axis::Horizontal => self.width,
            Axis::Vertical => self.height,
        }
    }

    fn add_to_axis(&mut self, value: u32, axis: Axis) {
        match axis {
            Axis::Horizontal => self.width = self.width.map(|width| width + value),
            Axis::Vertical => self.height = self.height.map(|height| height + value),
        }
    }
}

impl Inset for IntrinsicSize {
    fn inset(&self, insets: Insets) -> Self {
        Self {
            width: self.width.map(|width| width - insets.left - insets.right),
            height: self
                .height
                .map(|height| height - insets.top - insets.bottom),
        }
    }

    fn outset(&self, insets: Insets) -> Self {
        Self {
            width: self.width.map(|width| width + insets.left + insets.right),
            height: self
                .height
                .map(|height| height + insets.top + insets.bottom),
        }
    }
}

impl From<Size> for IntrinsicSize {
    fn from(size: Size) -> Self {
        Self {
            width: Some(size.width),
            height: Some(size.height),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IntrinsicSize;
    use embedded_graphics::prelude::*;

    #[test]
    fn new() {
        assert_eq!(
            IntrinsicSize::new(Some(1), Some(2)),
            IntrinsicSize {
                width: Some(1),
                height: Some(2)
            }
        )
    }

    #[test]
    fn to_size() {
        assert_eq!(
            IntrinsicSize::new(Some(1), Some(2)).to_size(),
            Some(Size::new(1, 2))
        );
        assert_eq!(IntrinsicSize::new(Some(1), None).to_size(), None);
        assert_eq!(IntrinsicSize::new(None, Some(2)).to_size(), None);
        assert_eq!(IntrinsicSize::new(None, None).to_size(), None);
    }

    #[test]
    fn to_size_with_defaults() {
        assert_eq!(
            IntrinsicSize::new(Some(1), Some(2)).to_size_with_defaults(Size::new(3, 4)),
            Size::new(1, 2)
        );
        assert_eq!(
            IntrinsicSize::new(Some(1), None).to_size_with_defaults(Size::new(3, 4)),
            Size::new(1, 4)
        );
        assert_eq!(
            IntrinsicSize::new(None, Some(2)).to_size_with_defaults(Size::new(3, 4)),
            Size::new(3, 2)
        );
        assert_eq!(
            IntrinsicSize::new(None, None).to_size_with_defaults(Size::new(3, 4)),
            Size::new(3, 4)
        );
    }

    #[test]
    fn from_size() {
        let intrinsic_size: IntrinsicSize = Size::new(1, 2).into();
        assert_eq!(intrinsic_size, IntrinsicSize::new(Some(1), Some(2)));
    }

    mod axis_size {
        use super::super::*;

        #[test]
        fn for_axis() {
            let intrinsic_size = IntrinsicSize::new(Some(1), Some(2));
            assert_eq!(intrinsic_size.for_axis(Axis::Horizontal), Some(1));
            assert_eq!(intrinsic_size.for_axis(Axis::Vertical), Some(2));
        }

        #[test]
        fn add_to_axis() {
            let mut intrinsic_size = IntrinsicSize::new(Some(1), Some(2));

            intrinsic_size.add_to_axis(10, Axis::Horizontal);
            assert_eq!(intrinsic_size, IntrinsicSize::new(Some(11), Some(2)));

            intrinsic_size.add_to_axis(10, Axis::Vertical);
            assert_eq!(intrinsic_size, IntrinsicSize::new(Some(11), Some(12)));

            let mut intrinsic_size = IntrinsicSize::none();

            intrinsic_size.add_to_axis(10, Axis::Horizontal);
            assert_eq!(intrinsic_size, IntrinsicSize::none());

            intrinsic_size.add_to_axis(10, Axis::Vertical);
            assert_eq!(intrinsic_size, IntrinsicSize::none());
        }
    }
}
