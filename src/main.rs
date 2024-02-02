// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::*;

mod consts;
use consts::*;

fn main() -> Result<(), eframe::Error> {
    let viewport = ViewportBuilder::default()
        .with_inner_size([320.0, 240.0])
        // .with_always_on_top()
        .with_decorations(false)
        .with_transparent(true);

    let options = eframe::NativeOptions {
        viewport,
        vsync: true,
        ..Default::default()
    };

    eframe::run_native("god", options, Box::new(|_cc| Box::<MyApp>::default()))
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals {
            resize_corner_size: 4.0,
            hyperlink_color: ThemeColors::BG_PURPLE_LIGHT,
            faint_bg_color: ThemeColors::BG_PURPLE_LIGHT,
            extreme_bg_color: ThemeColors::BG_PURPLE_DARK,

            widgets: style::Widgets {
                noninteractive: {
                    style::WidgetVisuals {
                        bg_fill: ThemeColors::BG_PURPLE_DEEP,
                        weak_bg_fill: ThemeColors::BG_PURPLE_DEEP,
                        bg_stroke: Stroke::new(2.0, ThemeColors::BG_PURPLE),
                        fg_stroke: Stroke::new(1.0, ThemeColors::TEXT),
                        rounding: Rounding::same(4.0),
                        expansion: 0.0,
                    }
                },

                ..style::Widgets::dark()
            },

            ..Visuals::dark()
        });

        let titlebar_text = format!("god v{}", APP_VERSION);
        custom_window_frame(ctx, &titlebar_text, |ui| {
            ui.heading(&titlebar_text);
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

fn custom_window_frame(ctx: &Context, title: &str, add_contents: impl FnOnce(&mut Ui)) {
    let vis = &ctx.style().visuals;
    let text_color = vis.text_color();
    let window_stroke = vis.window_stroke();
    let window_fill = vis.window_fill();

    CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();

            // Paint the frame:
            painter.rect(rect.shrink(1.0), 10.0, window_fill, window_stroke);

            // Paint the title:
            painter.text(
                rect.center_top() + vec2(0.0, TITLEBAR_HEIGHT / 2.0),
                Align2::CENTER_CENTER,
                title,
                FontId::proportional(TITLEBAR_HEIGHT * 0.8),
                text_color,
            );

            // Paint the line under the title:
            painter.line_segment(
                [
                    rect.left_top() + vec2(2.0, TITLEBAR_HEIGHT),
                    rect.right_top() + vec2(-2.0, TITLEBAR_HEIGHT),
                ],
                window_stroke,
            );

            // Add the close button:
            let close_button = ui.put(
                Rect::from_min_size(
                    rect.right_top() - vec2(32.0, 0.0),
                    Vec2::splat(TITLEBAR_HEIGHT),
                ),
                Button::new(RichText::new("❌").size(TITLEBAR_HEIGHT - 4.0)).frame(false),
            );

            if close_button.clicked() {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }

            // Draggable title bar
            let title_bar_rect = {
                let mut rect = rect;
                rect.max.y = rect.min.y + TITLEBAR_HEIGHT;
                rect
            };
            let title_bar_response =
                ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
            if title_bar_response.is_pointer_button_down_on() {
                ctx.send_viewport_cmd(ViewportCommand::StartDrag);
            }

            // Add the contents:
            let content_rect = {
                let mut rect = rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
            .shrink(4.0);
            let mut content_ui = ui.child_ui(content_rect, *ui.layout());
            add_contents(&mut content_ui);
        });
}
