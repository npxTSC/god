// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(lazy_cell)]

use std::sync::LazyLock;

use eframe::egui::*;

mod gui_consts;

mod custom_frame;
use custom_frame::*;

const APP_VER: &str = env!("CARGO_PKG_VERSION");
static TITLE_TEXT: LazyLock<String> = LazyLock::new(|| format!("god v{}", APP_VER));

pub fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_decorations(false)
            .with_inner_size([640.0, 480.0])
            .with_transparent(true),

        vsync: true,
        ..Default::default()
    };

    eframe::run_native(
        &TITLE_TEXT,
        options,
        Box::new(|_cc| Box::<GodGUI>::default()),
    )
}

#[derive(Default)]
struct GodGUI {}

impl eframe::App for GodGUI {
    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        set_purple_theme(ctx);

        custom_window_frame(ctx, &TITLE_TEXT, |_ui| {
            //
        });
    }
}
