use crate::{Cells, Cutout, Metric, SceneFragment};

/// TODO: docs
#[derive(Debug, Copy, Clone)]
pub struct ExpandRect<T: Metric> {
    pub(crate) top: T,
    pub(crate) bottom: T,
    pub(crate) left: T,
    pub(crate) right: T,
}

impl Default for ExpandRect<Cells> {
    #[inline]
    fn default() -> Self {
        Self::new(Cells::zero(), Cells::zero(), Cells::zero(), Cells::zero())
    }
}

impl<T: Metric> ExpandRect<T> {
    /// Creates a new [`ExpandRect`] with the given top, bottom, left, and right values.
    #[inline]
    pub fn new(top: T, bottom: T, left: T, right: T) -> Self {
        Self { top, bottom, left, right }
    }

    /// Sets the left and right edges of the [`ExpandRect`] to the given value.
    #[inline]
    pub fn x(mut self, expand_x_by: T) -> Self {
        self.left = expand_x_by;
        self.right = expand_x_by;
        self
    }

    /// Sets the top and bottom edges of the [`ExpandRect`] to the given value.
    #[inline]
    pub fn y(mut self, expand_y_by: T) -> Self {
        self.top = expand_y_by;
        self.bottom = expand_y_by;
        self
    }
}

impl Cutout for ExpandRect<Cells> {
    type Cutout = ExpandRectCutout;

    #[inline]
    fn cutout(self, _fragment: &mut SceneFragment) -> Self::Cutout {
        todo!();
    }
}

/// TODO: docs.
pub enum ExpandRectCutout {
    Single,
}
