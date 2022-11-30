use std::rc::Rc;
use eframe::{egui::{CentralPanel, ScrollArea, Grid, Button, TopBottomPanel, Layout}, NativeOptions, epaint::{Vec2}, App};
use egui_dock::DockArea;

pub mod ui;
use crate::ui::MainWindow;

pub mod data;
use crate::data::AdventureProject;

fn main() {
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(1600f32, 900f32));
    eframe::run_native("Adventu-rs", win_options, Box::new(|cc| Box::new(AdventureApp::new(cc))));
}

struct AdventureApp {
    pub main_window: MainWindow,
    pub project: Rc<AdventureProject>,
}

impl AdventureApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let proj = Rc::new(AdventureProject::new(String::from("Unnamed Adventure")));
        Self {
            main_window: MainWindow::new(proj.clone()),
            project: proj.clone(),
        }
    }
}

impl App for AdventureApp {

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.main_window.update(ctx, _frame);
    }

}
