use egui::{Pos2, Rect, Vec2};

/// Anchor where to show toasts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    /// Top right corner.
    TopRight,
    /// Top left corner.
    TopLeft,
    /// Bottom right corner.
    BottomRight,
    /// Bottom left corner
    BottomLeft,
}

impl Anchor {
    #[inline]
    pub(crate) const fn anim_side(&self) -> f32 {
        match self {
            Self::TopRight | Self::BottomRight => 1.,
            Self::TopLeft | Self::BottomLeft => -1.,
        }
    }
}

impl Anchor {
    pub(crate) fn rect_corner(&self, rect: Rect, margin: Vec2) -> Pos2 {
        let mut out = match self {
            Anchor::TopRight => rect.right_top(),
            Anchor::TopLeft => rect.left_top(),
            Anchor::BottomRight => rect.right_bottom(),
            Anchor::BottomLeft => rect.left_bottom(),
        };
        self.apply_margin(&mut out, margin);
        out
    }

    pub(crate) fn apply_margin(&self, pos: &mut Pos2, margin: Vec2) {
        match self {
            Self::TopRight => {
                pos.x -= margin.x;
                pos.y += margin.y;
            }
            Self::TopLeft => {
                pos.x += margin.x;
                pos.y += margin.y;
            }
            Self::BottomRight => {
                pos.x -= margin.x;
                pos.y -= margin.y;
            }
            Self::BottomLeft => {
                pos.x += margin.x;
                pos.y -= margin.y;
            }
        }
    }
}
