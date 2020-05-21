#![deny(warnings)]
#![warn(clippy::all)]

use std::sync::Arc;

use {
    emigui::{
        color::srgba, examples::ExampleApp, label, widgets::Separator, Align, RawInput, TextStyle,
        *,
    },
    emigui_wasm::now_sec,
};

use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Default, serde_derive::Deserialize)]
#[serde(default)]
struct WebInput {
    emigui: RawInput,
    web: Web,
}

#[derive(Clone, Debug, Default, serde_derive::Deserialize)]
#[serde(default)]
pub struct Web {
    pub location: String,
    /// i.e. "#fragment"
    pub location_hash: String,
}

#[wasm_bindgen]
pub struct State {
    example_app: ExampleApp,
    ctx: Arc<Context>,
    webgl_painter: emigui_wasm::webgl::Painter,

    frame_times: emigui::MovementTracker<f32>,
}

impl State {
    fn new(canvas_id: &str, pixels_per_point: f32) -> Result<State, JsValue> {
        let ctx = Context::new(pixels_per_point);
        emigui_wasm::load_memory(&ctx);
        Ok(State {
            example_app: Default::default(),
            ctx,
            webgl_painter: emigui_wasm::webgl::Painter::new(canvas_id)?,
            frame_times: emigui::MovementTracker::new(1000, 1.0),
        })
    }

    fn run(&mut self, web_input: WebInput) -> Result<Output, JsValue> {
        let everything_start = now_sec();

        self.ctx.begin_frame(web_input.emigui);

        let mut ui = self.ctx.fullscreen_ui();
        self.example_app.ui(&mut ui, &web_input.web.location_hash);
        let mut ui = ui.centered_column(ui.available().width().min(480.0));
        ui.set_layout(Layout::vertical(Align::Min));
        ui.add(label!("Emigui!").text_style(TextStyle::Heading));
        ui.add_label("Emigui is an immediate mode GUI written in Rust, compiled to WebAssembly, rendered with WebGL.");
        ui.add_label(
            "Everything you see is rendered as textured triangles. There is no DOM. There are no HTML elements."
        );
        ui.add_label("This is not JavaScript. This is Rust, running at 60 FPS. This is the web page, reinvented with game tech.");
        ui.add_label("This is also work in progress, and not ready for production... yet :)");
        ui.horizontal(|ui| {
            ui.add_label("Project home page:");
            ui.add_hyperlink("https://github.com/emilk/emigui/");
        });
        ui.add(Separator::new());

        ui.add_label("WebGl painter info:");
        ui.indent("webgl region id", |ui| {
            ui.add_label(self.webgl_painter.debug_info());
        });

        ui.add(
            label!(
                "CPU usage: {:.2} ms (excludes painting)",
                1e3 * self.frame_times.average().unwrap_or_default()
            )
            .text_style(TextStyle::Monospace),
        );
        ui.add(
            label!(
                "FPS: {:.1}",
                1.0 / self.frame_times.mean_time_interval().unwrap_or_default()
            )
            .text_style(TextStyle::Monospace),
        );

        let bg_color = srgba(0, 0, 0, 0); // Use background css color.
        let (output, batches) = self.ctx.end_frame();

        let now = now_sec();
        self.frame_times.add(now, (now - everything_start) as f32);

        self.webgl_painter.paint_batches(
            bg_color,
            batches,
            self.ctx.texture(),
            self.ctx.pixels_per_point(),
        )?;

        emigui_wasm::save_memory(&self.ctx); // TODO: don't save every frame

        Ok(output)
    }
}

#[wasm_bindgen]
pub fn new_webgl_gui(canvas_id: &str, pixels_per_point: f32) -> Result<State, JsValue> {
    State::new(canvas_id, pixels_per_point)
}

#[wasm_bindgen]
pub fn run_gui(state: &mut State, web_input_json: &str) -> Result<String, JsValue> {
    // TODO: nicer interface than JSON
    let web_input: WebInput = serde_json::from_str(web_input_json).unwrap();
    let output = state.run(web_input)?;
    Ok(serde_json::to_string(&output).unwrap())
}
