use eframe::{egui::{CentralPanel, ScrollArea, Grid, Button, TopBottomPanel, Layout}, NativeOptions, epaint::{Vec2}, App};

fn main() {
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(800f32, 600f32));
    eframe::run_native("Adventu-rs", win_options, Box::new(|cc| Box::new(UpdaterApp::new(cc))));
}

#[derive(Default)]
struct UpdaterApp {}

impl UpdaterApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl App for UpdaterApp {

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        
        CentralPanel::default().show(ctx, |ui| {
            //self.handle_package_grid(ui);
        });

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                            
            });
        });
    }
}
