use crate::*;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub(crate) struct State {
    /// This is the size that the user has picked by dragging the resize handles.
    /// This may be smaller and/or larger than the actual size.
    /// For instance, the user may have tried to shrink too much (not fitting the contents).
    /// Or the user requested a large area, but the content don't need that much space.
    pub(crate) desired_size: Vec2,

    /// Actual size of content last frame
    last_content_size: Vec2,

    /// Externally requested size (e.g. by Window) for the next frame
    pub(crate) requested_size: Option<Vec2>,
}

/// A region that can be resized by dragging the bottom right corner.
#[derive(Clone, Copy, Debug)]
pub struct Resize {
    id: Option<Id>,

    /// If false, we are no enabled
    resizable: bool,

    pub(crate) min_size: Vec2,

    default_size: Vec2,

    with_stroke: bool,
}

impl Default for Resize {
    fn default() -> Self {
        Self {
            id: None,
            resizable: true,
            min_size: Vec2::splat(16.0),
            default_size: vec2(320.0, 128.0), // TODO: preferred size of `Resize` area.
            with_stroke: true,
        }
    }
}

impl Resize {
    /// Assign an explicit and globally unique id.
    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    /// Preferred / suggested width. Actual width will depend on contents.
    ///
    /// Examples:
    /// * if the contents is text, this will decide where we break long lines.
    /// * if the contents is a canvas, this decides the width of it,
    /// * if the contents is some buttons, this is ignored and we will auto-size.
    pub fn default_width(mut self, width: f32) -> Self {
        self.default_size.x = width;
        self
    }

    /// Preferred / suggested height. Actual height will depend on contents.
    ///
    /// Examples:
    /// * if the contents is a `ScrollArea` then this decides the maximum size.
    /// * if the contents is a canvas, this decides the height of it,
    /// * if the contents is text and buttons, then the `default_height` is ignored
    ///   and the height is picked automatically..
    pub fn default_height(mut self, height: f32) -> Self {
        self.default_size.y = height;
        self
    }

    pub fn default_size(mut self, default_size: impl Into<Vec2>) -> Self {
        self.default_size = default_size.into();
        self
    }

    /// Won't shrink to smaller than this
    pub fn min_size(mut self, min_size: impl Into<Vec2>) -> Self {
        self.min_size = min_size.into();
        self
    }

    /// Can you resize it with the mouse?
    /// Note that a window can still auto-resize
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn is_resizable(&self) -> bool {
        self.resizable
    }

    /// Not manually resizable, just takes the size of its contents.
    pub fn auto_sized(self) -> Self {
        self.min_size(Vec2::zero())
            .default_size(Vec2::splat(f32::INFINITY))
            .resizable(false)
    }

    pub fn fixed_size(mut self, size: impl Into<Vec2>) -> Self {
        let size = size.into();
        self.default_size = size;
        self.min_size = size;
        self.resizable = false;
        self
    }

    pub fn with_stroke(mut self, with_stroke: bool) -> Self {
        self.with_stroke = with_stroke;
        self
    }
}

struct Prepared {
    id: Id,
    state: State,
    corner_response: Option<Response>,
    content_ui: Ui,
}

impl Resize {
    fn begin(&mut self, ui: &mut Ui) -> Prepared {
        let id = self.id.unwrap_or_else(|| ui.make_child_id("resize"));

        let mut state = ui.memory().resize.get(&id).cloned().unwrap_or_else(|| {
            ui.ctx().request_repaint(); // counter frame delay

            let default_size = self.default_size.max(self.min_size);

            State {
                desired_size: default_size,
                last_content_size: vec2(0.0, 0.0),
                requested_size: None,
            }
        });

        state.desired_size = state.desired_size.max(self.min_size);

        let position = ui.available().min;

        let corner_response = if self.resizable {
            // Resize-corner:
            let corner_size = Vec2::splat(ui.style().visuals.resize_corner_size);
            let corner_rect =
                Rect::from_min_size(position + state.desired_size - corner_size, corner_size);
            let corner_response = ui.interact(corner_rect, id.with("corner"), Sense::drag());

            if corner_response.active {
                if let Some(mouse_pos) = ui.input().mouse.pos {
                    state.desired_size = mouse_pos - position + 0.5 * corner_response.rect.size();
                }
            }
            Some(corner_response)
        } else {
            None
        };

        if let Some(requested_size) = state.requested_size.take() {
            state.desired_size = requested_size;
        }
        state.desired_size = state.desired_size.max(self.min_size);

        // ------------------------------

        let inner_rect = Rect::from_min_size(position, state.desired_size);

        let mut content_clip_rect = inner_rect.expand(ui.style().visuals.clip_rect_margin);

        // If we pull the resize handle to shrink, we want to TRY to shrink it.
        // After laying out the contents, we might be much bigger.
        // In those cases we don't want the clip_rect to be smaller, because
        // then we will clip the contents of the region even thought the result gets larger. This is simply ugly!
        // So we use the memory of last_content_size to make the clip rect large enough.
        content_clip_rect.max = content_clip_rect.max.max(
            inner_rect.min
                + state.last_content_size
                + Vec2::splat(ui.style().visuals.clip_rect_margin),
        );

        content_clip_rect = content_clip_rect.intersect(ui.clip_rect()); // Respect parent region

        let mut content_ui = ui.child_ui(inner_rect);
        content_ui.set_clip_rect(content_clip_rect);

        Prepared {
            id,
            state,
            corner_response,
            content_ui,
        }
    }

    pub fn show<R>(mut self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
        let mut prepared = self.begin(ui);
        let ret = add_contents(&mut prepared.content_ui);
        self.end(ui, prepared);
        ret
    }

    fn end(self, ui: &mut Ui, prepared: Prepared) {
        let Prepared {
            id,
            mut state,
            corner_response,
            content_ui,
        } = prepared;

        state.last_content_size = content_ui.bounding_size();
        state.last_content_size = state.last_content_size.ceil(); // Avoid rounding errors in math

        // ------------------------------

        if self.with_stroke || self.resizable {
            // We show how large we are,
            // so we must follow the contents:

            state.desired_size = state.desired_size.max(state.last_content_size);
            state.desired_size = ui.painter().round_vec_to_pixels(state.desired_size);

            // We are as large as we look
            ui.allocate_space(state.desired_size);
        } else {
            // Probably a window.
            ui.allocate_space(state.last_content_size);
        }

        // ------------------------------

        if self.with_stroke && corner_response.is_some() {
            let rect = Rect::from_min_size(content_ui.top_left(), state.desired_size);
            let rect = rect.expand(2.0); // breathing room for content
            ui.painter().add(paint::PaintCmd::Rect {
                rect,
                corner_radius: 3.0,
                fill: Default::default(),
                stroke: ui.style().visuals.thin_stroke,
            });
        }

        if let Some(corner_response) = corner_response {
            paint_resize_corner(ui, &corner_response);

            if corner_response.hovered || corner_response.active {
                ui.ctx().output().cursor_icon = CursorIcon::ResizeNwSe;
            }
        }

        ui.memory().resize.insert(id, state);

        if ui.ctx().style().visuals.debug_resize {
            ui.ctx().debug_painter().debug_rect(
                Rect::from_min_size(content_ui.top_left(), state.desired_size),
                color::GREEN,
                "desired_size",
            );
            ui.ctx().debug_painter().debug_rect(
                Rect::from_min_size(content_ui.top_left(), state.last_content_size),
                color::LIGHT_BLUE,
                "last_content_size",
            );
        }
    }
}

use crate::paint::Stroke;

pub fn paint_resize_corner(ui: &mut Ui, response: &Response) {
    let stroke = ui.style().interact(response).stroke;
    paint_resize_corner_with_style(ui, &response.rect, stroke);
}

pub fn paint_resize_corner_with_style(ui: &mut Ui, rect: &Rect, stroke: Stroke) {
    let painter = ui.painter();
    let corner = painter.round_pos_to_pixels(rect.right_bottom());
    let mut w = 2.0;

    while w <= rect.width() && w <= rect.height() {
        painter.line_segment(
            [pos2(corner.x - w, corner.y), pos2(corner.x, corner.y - w)],
            stroke,
        );
        w += 4.0;
    }
}
