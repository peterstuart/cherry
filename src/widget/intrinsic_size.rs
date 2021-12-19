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
}
