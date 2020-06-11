//! Egui tracks widgets frame-to-frame using `Id`s.
//!
//! For instance, if you start dragging a slider one frame, egui stores
//! the sliders `Id` as the current active id so that next frame when
//! you move the mouse the same slider changes, even if the mouse has
//! moved outside the slider.
//!
//! For some widgets `Id`s are also used to persist some state about the
//! widgets, such as Window position or wether not a collapsing header region is open.
//!
//! This implicates that the `Id`s must be unqiue.
//!
//! For simple things like sliders and buttons that don't have any memory and
//! doesn't move we can use the location of the widget as a source of identity.
//! For instance, a slider only needs a unique and persistent ID while you are
//! dragging the slider. As long as it is still while moving, that is fine.
//!
//! For things that need to persist state even after moving (windows, collapsing headers)
//! the location of the widgets is obviously not good enough. For instance,
//! a collapsing region needs to remember wether or not it is open even
//! if the layout next frame is different and the collapsing is not lower down
//! on the screen.
//!
//! Then there are widgets that need no identifiers at all, like labels,
//! because they have no state nor are interacted with.
//!
//! So we have two type of Ids: `PositionId` and `UniqueId`.
//! TODO: have separate types for `PositionId` and `UniqueId`.

use std::hash::Hash;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Id(u64);

impl Id {
    pub fn background() -> Self {
        Self(0)
    }

    pub fn tooltip() -> Self {
        Self(1)
    }

    pub fn new(source: impl Hash) -> Id {
        use std::hash::Hasher;
        let mut hasher = ahash::AHasher::default();
        source.hash(&mut hasher);
        Id(hasher.finish())
    }

    pub fn with(self, child: impl Hash) -> Id {
        use std::hash::Hasher;
        let mut hasher = ahash::AHasher::default();
        hasher.write_u64(self.0);
        child.hash(&mut hasher);
        Id(hasher.finish())
    }
}
