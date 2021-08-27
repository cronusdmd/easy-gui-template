#[derive(Default)]
pub struct CursorTest {}

impl super::Demo for CursorTest {
    fn name(&self) -> &'static str {
        "Cursor Test"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        egui::Window::new(self.name()).open(open).show(ctx, |ui| {
            use super::View;
            self.ui(ui);
        });
    }
}

impl super::View for CursorTest {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.heading("Hover to switch cursor icon:");
            for &cursor_icon in &egui::CursorIcon::ALL {
                let _ = ui
                    .button(format!("{:?}", cursor_icon))
                    .on_hover_cursor(cursor_icon);
            }
            ui.add(crate::__egui_github_link_file!());
        });
    }
}

// ----------------------------------------------------------------------------

#[derive(Default)]
pub struct IdTest {}

impl super::Demo for IdTest {
    fn name(&self) -> &'static str {
        "ID Test"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        egui::Window::new(self.name()).open(open).show(ctx, |ui| {
            use super::View;
            self.ui(ui);
        });
    }
}

impl super::View for IdTest {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Name collision example");

        ui.label("\
            Widgets that store state require unique and persisting identifiers so we can track their state between frames.\n\
            For instance, collapsable headers needs to store wether or not they are open. \
            Their Id:s are derived from their names. \
            If you fail to give them unique names then clicking one will open both. \
            To help you debug this, an error message is printed on screen:");

        ui.collapsing("Collapsing header", |ui| {
            ui.label("Contents of first foldable ui");
        });
        ui.collapsing("Collapsing header", |ui| {
            ui.label("Contents of second foldable ui");
        });

        ui.label("\
            Any widget that can be interacted with also need a unique Id. \
            For most widgets the Id is generated by a running counter. \
            As long as elements are not added or removed, the Id stays the same. \
            This is fine, because during interaction (i.e. while dragging a slider), \
            the number of widgets previously in the same window is most likely not changing \
            (and if it is, the window will have a new layout, and the slider will endup somewhere else, and so aborthing the interaction probably makes sense).");

        ui.label("So these buttons have automatic Id:s, and therefore there is no name clash:");
        let _ = ui.button("Button");
        let _ = ui.button("Button");

        ui.vertical_centered(|ui| {
            ui.add(crate::__egui_github_link_file!());
        });
    }
}

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
enum WidgetType {
    Label,
    Button,
    TextEdit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ManualLayoutTest {
    widget_offset: egui::Vec2,
    widget_size: egui::Vec2,
    widget_type: WidgetType,
    text_edit_contents: String,
}

impl Default for ManualLayoutTest {
    fn default() -> Self {
        Self {
            widget_offset: egui::Vec2::splat(150.0),
            widget_size: egui::Vec2::new(200.0, 100.0),
            widget_type: WidgetType::Button,
            text_edit_contents: crate::LOREM_IPSUM.to_owned(),
        }
    }
}

impl super::Demo for ManualLayoutTest {
    fn name(&self) -> &'static str {
        "Manual Layout Test"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        egui::Window::new(self.name())
            .resizable(false)
            .open(open)
            .show(ctx, |ui| {
                use super::View;
                self.ui(ui);
            });
    }
}

impl super::View for ManualLayoutTest {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::reset_button(ui, self);

        let Self {
            widget_offset,
            widget_size,
            widget_type,
            text_edit_contents,
        } = self;
        ui.horizontal(|ui| {
            ui.label("Test widget:");
            ui.radio_value(widget_type, WidgetType::Button, "Button");
            ui.radio_value(widget_type, WidgetType::Label, "Label");
            ui.radio_value(widget_type, WidgetType::TextEdit, "TextEdit");
        });
        egui::Grid::new("pos_size").show(ui, |ui| {
            ui.label("Widget position:");
            ui.add(egui::Slider::new(&mut widget_offset.x, 0.0..=400.0));
            ui.add(egui::Slider::new(&mut widget_offset.y, 0.0..=400.0));
            ui.end_row();

            ui.label("Widget size:");
            ui.add(egui::Slider::new(&mut widget_size.x, 0.0..=400.0));
            ui.add(egui::Slider::new(&mut widget_size.y, 0.0..=400.0));
            ui.end_row();
        });

        let widget_rect =
            egui::Rect::from_min_size(ui.min_rect().min + *widget_offset, *widget_size);

        ui.add(crate::__egui_github_link_file!());

        // Showing how to place a widget anywhere in the `Ui`:
        match *widget_type {
            WidgetType::Button => {
                ui.put(widget_rect, egui::Button::new("Example button"));
            }
            WidgetType::Label => {
                ui.put(widget_rect, egui::Label::new("Example label"));
            }
            WidgetType::TextEdit => {
                ui.put(widget_rect, egui::TextEdit::multiline(text_edit_contents));
            }
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(PartialEq)]
pub struct TableTest {
    num_cols: usize,
    num_rows: usize,
    min_col_width: f32,
    max_col_width: f32,
    text_length: usize,
}

impl Default for TableTest {
    fn default() -> Self {
        Self {
            num_cols: 4,
            num_rows: 4,
            min_col_width: 10.0,
            max_col_width: 200.0,
            text_length: 10,
        }
    }
}

impl super::Demo for TableTest {
    fn name(&self) -> &'static str {
        "Table Test"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        egui::Window::new(self.name()).open(open).show(ctx, |ui| {
            use super::View;
            self.ui(ui);
        });
    }
}

impl super::View for TableTest {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.add(
            egui::Slider::new(&mut self.min_col_width, 0.0..=400.0).text("Minimum column width"),
        );
        ui.add(
            egui::Slider::new(&mut self.max_col_width, 0.0..=400.0).text("Maximum column width"),
        );
        ui.add(egui::Slider::new(&mut self.num_cols, 0..=5).text("Columns"));
        ui.add(egui::Slider::new(&mut self.num_rows, 0..=20).text("Rows"));

        ui.separator();

        let words = [
            "random", "words", "in", "a", "random", "order", "that", "just", "keeps", "going",
            "with", "some", "more",
        ];

        egui::Grid::new("my_grid")
            .striped(true)
            .min_col_width(self.min_col_width)
            .max_col_width(self.max_col_width)
            .show(ui, |ui| {
                for row in 0..self.num_rows {
                    for col in 0..self.num_cols {
                        if col == 0 {
                            ui.label(format!("row {}", row));
                        } else {
                            let word_idx = row * 3 + col * 5;
                            let word_count = (row * 5 + col * 75) % 13;
                            let mut string = String::new();
                            for word in words.iter().cycle().skip(word_idx).take(word_count) {
                                string += word;
                                string += " ";
                            }
                            ui.label(string);
                        }
                    }
                    ui.end_row();
                }
            });

        ui.separator();
        ui.add(egui::Slider::new(&mut self.text_length, 1..=40).text("Text length"));
        egui::Grid::new("parent grid").striped(true).show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label("Vertical nest1");
                ui.label("Vertical nest2");
            });
            ui.label("First row, second column");
            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Horizontal nest1");
                ui.label("Horizontal nest2");
            });
            ui.label("Second row, second column");
            ui.end_row();

            ui.scope(|ui| {
                ui.label("Scope nest 1");
                ui.label("Scope nest 2");
            });
            ui.label("Third row, second column");
            ui.end_row();

            egui::Grid::new("nested grid").show(ui, |ui| {
                ui.label("Grid nest11");
                ui.label("Grid nest12");
                ui.end_row();
                ui.label("Grid nest21");
                ui.label("Grid nest22");
                ui.end_row();
            });
            ui.label("Fourth row, second column");
            ui.end_row();

            let mut dyn_text = String::from("O");
            dyn_text.extend(std::iter::repeat('h').take(self.text_length));
            ui.label(dyn_text);
            ui.label("Fifth row, second column");
            ui.end_row();
        });

        ui.vertical_centered(|ui| {
            egui::reset_button(ui, self);
            ui.add(crate::__egui_github_link_file!());
        });
    }
}

// ----------------------------------------------------------------------------

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Default)]
pub struct InputTest {
    info: String,
}

impl super::Demo for InputTest {
    fn name(&self) -> &'static str {
        "Input Test"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable(false)
            .show(ctx, |ui| {
                use super::View;
                self.ui(ui);
            });
    }
}

impl super::View for InputTest {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add(crate::__egui_github_link_file!());
        });

        let response = ui.add(
            egui::Button::new("Click, double-click or drag me with any mouse button")
                .sense(egui::Sense::click_and_drag()),
        );

        let mut new_info = String::new();
        for &button in &[
            egui::PointerButton::Primary,
            egui::PointerButton::Secondary,
            egui::PointerButton::Middle,
        ] {
            if response.clicked_by(button) {
                new_info += &format!("Clicked by {:?} button\n", button);
            }
            if response.double_clicked_by(button) {
                new_info += &format!("Double-clicked by {:?} button\n", button);
            }
            if response.dragged_by(button) {
                new_info += &format!(
                    "Dragged by {:?} button, delta: {:?}\n",
                    button,
                    response.drag_delta()
                );
            }
        }
        if !new_info.is_empty() {
            self.info = new_info;
        }

        ui.label(&self.info);
    }
}

// ----------------------------------------------------------------------------

pub struct WindowResizeTest {
    text: String,
}

impl Default for WindowResizeTest {
    fn default() -> Self {
        Self {
            text: crate::LOREM_IPSUM.to_owned(),
        }
    }
}

impl super::Demo for WindowResizeTest {
    fn name(&self) -> &'static str {
        "↔ Window Resize"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        use egui::*;

        Window::new("↔ auto-sized")
            .open(open)
            .auto_sized()
            .show(ctx, |ui| {
                ui.label("This window will auto-size based on its contents.");
                ui.heading("Resize this area:");
                Resize::default().show(ui, |ui| {
                    ui.code(crate::LOREM_IPSUM);
                });
                ui.heading("Resize the above area!");
            });

        Window::new("↔ resizable + scroll")
            .open(open)
            .scroll(true)
            .resizable(true)
            .default_height(300.0)
            .show(ctx, |ui| {
                ui.label(
                    "This window is resizable and has a scroll area. You can shrink it to any size",
                );
                ui.separator();
                ui.code(crate::LOREM_IPSUM_LONG);
            });

        Window::new("↔ resizable + embedded scroll")
            .open(open)
            .scroll(false)
            .resizable(true)
            .default_height(300.0)
            .show(ctx, |ui| {
                ui.label("This window is resizable but has no built-in scroll area.");
                ui.label("However, we have a sub-region with a scroll bar:");
                ui.separator();
                ScrollArea::auto_sized().show(ui, |ui| {
                    ui.code(crate::LOREM_IPSUM_LONG);
                    ui.code(crate::LOREM_IPSUM_LONG);
                });
                // ui.heading("Some additional text here, that should also be visible"); // this works, but messes with the resizing a bit
            });

        Window::new("↔ resizable without scroll")
            .open(open)
            .scroll(false)
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("This window is resizable but has no scroll area. This means it can only be resized to a size where all the contents is visible.");
                ui.label("egui will not clip the contents of a window, nor add whitespace to it.");
                ui.separator();
                ui.code(crate::LOREM_IPSUM);
            });

        Window::new("↔ resizable with TextEdit")
            .open(open)
            .scroll(false)
            .resizable(true)
            .default_height(300.0)
            .show(ctx, |ui| {
                ui.label("Shows how you can fill an area with a widget.");
                ui.add_sized(ui.available_size(), TextEdit::multiline(&mut self.text));
            });

        Window::new("↔ freely resized")
            .open(open)
            .scroll(false)
            .resizable(true)
            .default_size([250.0, 150.0])
            .show(ctx, |ui| {
                ui.label("This window has empty space that fills up the available space, preventing auto-shrink.");
                ui.allocate_space(ui.available_size());
            });
    }
}
