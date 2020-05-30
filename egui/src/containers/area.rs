//! Area is a `Ui` that has no parent, it floats on the background.
//! It has no frame or own size. It is potentioally movable.
//! It is the foundation for windows and popups.

use std::{fmt::Debug, hash::Hash, sync::Arc};

use crate::*;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
pub(crate) struct State {
    /// Last known pos
    pub pos: Pos2,

    /// Last know size. Used for catching clicks.
    pub size: Vec2,

    /// If false, clicks goes stright throught to what is behind us.
    /// Good for tooltips etc.
    pub interactable: bool,

    /// You can throw a moveable Area. It's fun.
    /// TODO: separate out moveable to container?
    #[cfg_attr(feature = "with_serde", serde(skip))]
    pub vel: Vec2,
}

impl State {
    pub fn rect(&self) -> Rect {
        Rect::from_min_size(self.pos, self.size)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Area {
    id: Id,
    movable: bool,
    interactable: bool,
    order: Order,
    default_pos: Option<Pos2>,
    fixed_pos: Option<Pos2>,
}

impl Area {
    pub fn new(id_source: impl Hash) -> Self {
        Self {
            id: Id::new(id_source),
            movable: true,
            interactable: true,
            order: Order::Middle,
            default_pos: None,
            fixed_pos: None,
        }
    }

    pub fn layer(&self) -> Layer {
        Layer {
            order: self.order,
            id: self.id,
        }
    }

    /// moveable by draggin the area?
    pub fn movable(mut self, movable: bool) -> Self {
        self.movable = movable;
        self.interactable |= movable;
        self
    }

    pub fn is_movable(&self) -> bool {
        self.movable
    }

    /// If false, clicks goes stright throught to what is behind us.
    /// Good for tooltips etc.
    pub fn interactable(mut self, interactable: bool) -> Self {
        self.interactable = interactable;
        self.movable &= interactable;
        self
    }

    /// `order(Order::Foreground)` for an Area that should always be on top
    pub fn order(mut self, order: Order) -> Self {
        self.order = order;
        self
    }

    pub fn default_pos(mut self, default_pos: impl Into<Pos2>) -> Self {
        self.default_pos = Some(default_pos.into());
        self
    }

    pub fn fixed_pos(mut self, fixed_pos: impl Into<Pos2>) -> Self {
        let fixed_pos = fixed_pos.into();
        self.default_pos = Some(fixed_pos);
        self.fixed_pos = Some(fixed_pos);
        self.movable = false;
        self
    }
}

pub(crate) struct Prepared {
    layer: Layer,
    state: State,
    movable: bool,
}

impl Area {
    pub(crate) fn begin(self, ctx: &Arc<Context>) -> Prepared {
        let Area {
            id,
            movable,
            order,
            interactable,
            default_pos,
            fixed_pos,
        } = self;

        let default_pos = default_pos.unwrap_or_else(|| pos2(100.0, 100.0)); // TODO
        let id = ctx.register_unique_id(id, "Area", default_pos);
        let layer = Layer { order, id };

        let mut state = ctx.memory().areas.get(id).unwrap_or_else(|| State {
            pos: default_pos,
            size: Vec2::zero(),
            interactable,
            vel: Vec2::zero(),
        });
        state.pos = fixed_pos.unwrap_or(state.pos);
        state.pos = state.pos.round();

        Prepared {
            layer,
            state,
            movable,
        }
    }

    pub fn show(self, ctx: &Arc<Context>, add_contents: impl FnOnce(&mut Ui)) -> InteractInfo {
        let prepared = self.begin(ctx);
        let mut content_ui = prepared.content_ui(ctx);
        add_contents(&mut content_ui);
        prepared.end(ctx, content_ui)
    }
}

impl Prepared {
    pub(crate) fn state(&self) -> &State {
        &self.state
    }

    pub(crate) fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    pub(crate) fn content_ui(&self, ctx: &Arc<Context>) -> Ui {
        Ui::new(
            ctx.clone(),
            self.layer,
            self.layer.id,
            Rect::from_min_size(self.state.pos, Vec2::infinity()),
        )
    }

    pub(crate) fn end(self, ctx: &Arc<Context>, content_ui: Ui) -> InteractInfo {
        let Prepared {
            layer,
            mut state,
            movable,
        } = self;

        state.size = (content_ui.child_bounds().max - state.pos).ceil();

        let rect = Rect::from_min_size(state.pos, state.size);
        let clip_rect = Rect::everything(); // TODO: get from context

        let interact_id = if movable {
            Some(layer.id.with("move"))
        } else {
            None
        };
        let move_interact =
            ctx.interact(layer, clip_rect, rect, interact_id, Sense::click_and_drag());

        let input = ctx.input();
        if move_interact.active {
            state.pos += input.mouse.delta;
            state.vel = input.mouse.velocity;
        } else {
            let stop_speed = 20.0; // Pixels per second.
            let friction_coeff = 1000.0; // Pixels per second squared.

            let friction = friction_coeff * input.dt;
            if friction > state.vel.length() || state.vel.length() < stop_speed {
                state.vel = Vec2::zero();
            } else {
                state.vel -= friction * state.vel.normalized();
                state.pos += state.vel * input.dt;
            }
        }

        // Constrain to screen:
        let margin = 32.0;
        state.pos = state.pos.max(pos2(margin - state.size.x, 0.0));
        state.pos = state.pos.min(pos2(
            ctx.input().screen_size.x - margin,
            ctx.input().screen_size.y - margin,
        ));

        state.pos = state.pos.round();

        // ctx.debug_rect(
        //     Rect::from_min_size(state.pos, state.size),
        //     &format!("Area size: {:?}", state.size),
        // );

        if move_interact.active
            || mouse_pressed_on_area(ctx, layer)
            || !ctx.memory().areas.visible_last_frame(&layer)
        {
            ctx.memory().areas.move_to_top(layer);
        }
        ctx.memory().areas.set_state(layer, state);

        move_interact
    }
}

fn mouse_pressed_on_area(ctx: &Context, layer: Layer) -> bool {
    if let Some(mouse_pos) = ctx.input().mouse.pos {
        ctx.input().mouse.pressed && ctx.memory().layer_at(mouse_pos) == Some(layer)
    } else {
        false
    }
}
