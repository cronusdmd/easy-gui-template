use std::sync::Arc;

use crate::*;

/// Show a tooltip at the current mouse position (if any).
pub fn show_tooltip(ctx: &Arc<Context>, add_contents: impl FnOnce(&mut Ui)) {
    if let Some(mouse_pos) = ctx.input().mouse.pos {
        //  TODO: default size
        let id = Id::tooltip();
        let window_pos = mouse_pos + vec2(16.0, 16.0);
        show_popup(ctx, id, window_pos, add_contents);
    }
}

/// Show a tooltip at the current mouse position (if any).
pub fn show_tooltip_text(ctx: &Arc<Context>, text: impl Into<String>) {
    show_tooltip(ctx, |ui| {
        ui.add(crate::widgets::Label::new(text));
    })
}

/// Show a pop-over window.
fn show_popup(
    ctx: &Arc<Context>,
    id: Id,
    window_pos: Pos2,
    add_contents: impl FnOnce(&mut Ui),
) -> Response {
    use containers::*;
    Area::new(id)
        .order(Order::Tooltip)
        .fixed_pos(window_pos)
        .interactable(false)
        .show(ctx, |ui| Frame::popup(&ctx.style()).show(ui, add_contents))
}
