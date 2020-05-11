use std::sync::Arc;

use serde_derive::{Deserialize, Serialize};

use crate::{containers::*, widgets::*, *};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct FractalClock {
    paused: bool,
    time: f64,
    zoom: f32,
    start_line_width: f32,
    depth: usize,
    length_factor: f32,
    luminance_factor: f32,
    width_factor: f32,
}

impl Default for FractalClock {
    fn default() -> Self {
        Self {
            paused: false,
            time: 0.0,
            zoom: 0.25,
            start_line_width: 2.5,
            depth: 9,
            length_factor: 0.8,
            luminance_factor: 0.8,
            width_factor: 0.9,
        }
    }
}

impl FractalClock {
    pub fn window(&mut self, ctx: &Arc<Context>, open: &mut bool) {
        Window::new("FractalClock")
            .open(open)
            .default_rect(ctx.rect().expand(-40.0))
            .scroll(false)
            // Dark background frame to make it pop:
            .frame(Frame::window(&ctx.style()).fill_color(Some(color::black(250))))
            .show(ctx, |ui| self.ui(ui));
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        self.fractal_ui(ui);

        // TODO: background frame etc
        Frame::popup(ui.style())
            .fill_color(Some(color::gray(34, 160)))
            .outline(None)
            .show(&mut ui.left_column(320.0), |ui| self.options_ui(ui));
    }

    fn options_ui(&mut self, ui: &mut Ui) {
        let time = if let Some(seconds_since_midnight) = ui.input().seconds_since_midnight {
            ui.add(label!(
                "Local time: {:02}:{:02}:{:02}.{:03}",
                (seconds_since_midnight.rem_euclid(24.0 * 60.0 * 60.0) / 3600.0).floor(),
                (seconds_since_midnight.rem_euclid(60.0 * 60.0) / 60.0).floor(),
                (seconds_since_midnight.rem_euclid(60.0)).floor(),
                (seconds_since_midnight.rem_euclid(1.0) * 1000.0).floor()
            ));
            seconds_since_midnight
        } else {
            ui.add(label!(
                "The fractal_clock clock is not showing the correct time"
            ));
            ui.input().time
        };

        ui.add(Checkbox::new(&mut self.paused, "Paused"));
        ui.add(Slider::f32(&mut self.zoom, 0.0..=1.0).text("zoom"));
        ui.add(Slider::f32(&mut self.start_line_width, 0.0..=5.0).text("Start line width"));
        ui.add(Slider::usize(&mut self.depth, 0..=14).text("depth"));
        ui.add(Slider::f32(&mut self.length_factor, 0.0..=1.0).text("length factor"));
        ui.add(Slider::f32(&mut self.luminance_factor, 0.0..=1.0).text("luminance factor"));
        ui.add(Slider::f32(&mut self.width_factor, 0.0..=1.0).text("width factor"));
        if ui.add(Button::new("Reset")).clicked {
            *self = Default::default();
        }

        if !self.paused {
            self.time = time;
        }

        ui.add(
            Hyperlink::new("http://www.dqd.com/~mayoff/programs/FractalClock/")
                .text("Inspired by a screensaver by Rob Mayoff"),
        );
    }

    fn fractal_ui(&mut self, ui: &mut Ui) {
        struct Hand {
            length: f32,
            angle: f32,
            vec: Vec2,
        }

        impl Hand {
            fn from_length_angle(length: f32, angle: f32) -> Self {
                Self {
                    length,
                    angle,
                    vec: length * Vec2::angled(angle),
                }
            }
        }

        let angle_from_period =
            |period| TAU * (self.time.rem_euclid(period) / period) as f32 - TAU / 4.0;

        let hands = [
            // Second hand:
            Hand::from_length_angle(self.length_factor, angle_from_period(60.0)),
            // Minute hand:
            Hand::from_length_angle(self.length_factor, angle_from_period(60.0 * 60.0)),
            // Hour hand:
            Hand::from_length_angle(0.5, angle_from_period(12.0 * 60.0 * 60.0)),
        ];

        let rect = ui.available_rect_min();

        let scale = self.zoom * rect.width().min(rect.height());
        let mut paint_line = |points: [Pos2; 2], color: Color, width: f32| {
            let line = [
                rect.center() + scale * points[0].to_vec2(),
                rect.center() + scale * points[1].to_vec2(),
            ];

            ui.add_paint_cmd(PaintCmd::line_segment([line[0], line[1]], color, width));
        };

        let hand_rotations = [
            hands[0].angle - hands[2].angle,
            hands[1].angle - hands[2].angle,
        ];

        let hand_rotors = [
            hands[0].length * Vec2::angled(hand_rotations[0]),
            hands[1].length * Vec2::angled(hand_rotations[1]),
        ];

        #[derive(Clone, Copy)]
        struct Node {
            pos: Pos2,
            dir: Vec2,
        }

        let mut nodes = Vec::new();

        let mut width = self.start_line_width;

        for (i, hand) in hands.iter().enumerate() {
            let center = pos2(0.0, 0.0);
            let end = center + hand.vec;
            paint_line([center, end], color::additive_gray(255), width);
            if i < 2 {
                nodes.push(Node {
                    pos: end,
                    dir: hand.vec,
                });
            }
        }

        let mut luminance = 0.7; // Start dimmer than main hands

        let mut new_nodes = Vec::new();
        for _ in 0..self.depth {
            new_nodes.clear();
            new_nodes.reserve(nodes.len() * 2);

            luminance *= self.luminance_factor;
            width *= self.width_factor;

            let luminance_u8 = (255.0 * luminance).round() as u8;

            for rotor in &hand_rotors {
                for a in &nodes {
                    let new_dir = rotor.rotate_other(a.dir);
                    let b = Node {
                        pos: a.pos + new_dir,
                        dir: new_dir,
                    };
                    paint_line([a.pos, b.pos], color::additive_gray(luminance_u8), width);
                    new_nodes.push(b);
                }
            }

            std::mem::swap(&mut nodes, &mut new_nodes);
        }
    }
}
