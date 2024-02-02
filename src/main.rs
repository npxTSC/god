// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::*;

mod consts;
use consts::*;

fn main() -> Result<(), eframe::Error> {
    let viewport = ViewportBuilder::default()
        .with_inner_size([640.0, 480.0])
        .with_always_on_top()
        .with_transparent(true);

    let options = eframe::NativeOptions {
        viewport,
        vsync: true,
        ..Default::default()
    };

    eframe::run_native(
        &format!("god v{}", APP_VERSION),
        options,
        Box::new(|_cc| Box::<GodGUI>::default()),
    )
}

struct GodGUI {
    name: String,
}

impl Default for GodGUI {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
        }
    }
}

impl eframe::App for GodGUI {
    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        [0.0; 4]
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        use theme_colors::*;

        ctx.set_visuals(Visuals {
            resize_corner_size: 4.0,
            hyperlink_color: BG_PURPLE_LIGHT,
            faint_bg_color: BG_PURPLE_LIGHT,
            extreme_bg_color: BG_PURPLE_DARK,

            widgets: style::Widgets {
                noninteractive: {
                    style::WidgetVisuals {
                        bg_fill: BG_PURPLE_DEEP,
                        weak_bg_fill: BG_PURPLE_DEEP,
                        bg_stroke: Stroke::new(2.0, BG_PURPLE),
                        fg_stroke: Stroke::new(1.0, TEXT),
                        rounding: Rounding::same(4.0),
                        expansion: 0.0,
                    }
                },

                ..style::Widgets::dark()
            },

            ..Visuals::dark()
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.label(format!("Hello '{}'", self.name));
        });
    }
}
