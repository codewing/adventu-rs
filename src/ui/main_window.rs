use std::rc::Rc;
use eframe::egui::{Frame, TopBottomPanel};
use eframe::egui::style::Margin;
use egui_dock::{NodeIndex, DockArea, DynamicTree, TabBuilder};
use crate::AdventureProject;

use super::tab_example::{Editor};
use super::scene_view::{SimpleSceneView};

pub struct MainWindow {
    tree: DynamicTree
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

        Self { tree }
    }

    pub fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top")
            .frame(Frame::none().inner_margin(Margin::same(2.0)))
            .show(ctx, |ui| {
                if ui.button("Add Editor").clicked() {
                    self.tree
                        .push_to_focused_leaf(Box::new(Editor::new("New Text".into())));
                }
            });
        DockArea::new(&mut self.tree).show(ctx, &mut egui_dock::DynamicTabViewer {});
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