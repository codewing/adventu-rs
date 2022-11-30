use std::rc::Rc;
use eframe::egui::{Frame, TopBottomPanel};
use eframe::egui::style::Margin;
use eframe::emath::Align;
use egui_dock::{NodeIndex, DockArea, DynamicTree, TabBuilder};
use crate::AdventureProject;

use super::tab_example::{Editor};
use super::scene_view::{SimpleSceneView};

pub struct MainWindow {
    tree: DynamicTree,
    project: Rc<AdventureProject>,
    show_rename_dialog: bool,
}

impl MainWindow {
    pub fn new(project: Rc<AdventureProject>) -> Self {
        let tab1 = Box::new(Editor::new("Text".into()));

        let scene_view = Box::new(SimpleSceneView::new(project.scenes.clone()));

        let tab3 = TabBuilder::default()
            .title("Tab 3")
            .content(|ui| {
                ui.label("Tab 3");
            })
            .build();
        let tab4 = TabBuilder::default()
            .title("Tab 4")
            .content(|ui| {
                ui.label("Tab 4");
            })
            .build();
        let tab5 = TabBuilder::default()
            .title("Tab 5")
            .content(|ui| {
                ui.label("Tab 5");
            })
            .build();

        let mut tree = DynamicTree::new(vec![tab1, scene_view]);

        // You can modify the tree before constructing the dock
        let [a, b] = tree.split_left(NodeIndex::root(), 0.3, vec![tab3]);
        let [_, _] = tree.split_below(a, 0.7, vec![tab4]);
        let [_, _] = tree.split_below(b, 0.5, vec![tab5]);

        Self { tree, project, show_rename_dialog: false }
    }

    pub fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top")
            .frame(Frame::none().inner_margin(Margin::same(2.0)))
            .show(ctx, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open Project").clicked() {
                        
                    }

                    if ui.button("Rename Project").clicked() {
                        self.show_rename_dialog = true;
                    }
                    if ui.button("Save Project").clicked() {
                        self.show_rename_dialog = true;
                    }
                });
                ui.menu_button("View", |ui| {
                    
                });
            });
        DockArea::new(&mut self.tree).show(ctx, &mut egui_dock::DynamicTabViewer {});

        if self.show_rename_dialog {
            eframe::egui::Window::new("Rename Project")
                .collapsible(false)
                .resizable(false)
                .anchor(eframe::egui::Align2::CENTER_CENTER, eframe::egui::Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        let mut title = self.project.title.to_string();
                        ui.text_edit_singleline(&mut title);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_rename_dialog = false;
                        }

                        if ui.button("Rename").clicked() {
                            
                            self.show_rename_dialog = false;
                        }
                    });
                });
        }

    }
}

struct TabViewer {}

impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn ui(&mut self, ui: &mut eframe::egui::Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of {tab}"));
    }

    fn title(&mut self, tab: &mut Self::Tab) -> eframe::egui::WidgetText {
        (&*tab).into()
    }
}