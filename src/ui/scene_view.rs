use std::rc::Rc;
use eframe::egui::{Context, Ui, SidePanel, CentralPanel, Area, pos2, Frame, WidgetText, TextBuffer};
use eframe::egui::style::Margin;
use egui_dock::Tab;
use crate::data::scene::Scene;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SimpleSceneView {
    pub scenes: Vec<Rc<Scene>>,
    pub current_scene: Option<Rc<Scene>>,
}

impl SimpleSceneView {

    fn empty() -> Self {
        Self{
            scenes: vec![],
            current_scene: None
        }
    }

    pub fn new(scenes: Vec<Rc<Scene>>) -> Self {
        let first_scene;
        if scenes.len() > 0 {
            first_scene = Some(scenes[0].clone());
        } else {
            first_scene = None;
        }

        Self{
            scenes,
            current_scene: first_scene,
        }
    }
}

impl Tab for SimpleSceneView {
    fn ui(&mut self, ui: &mut Ui) {
        Frame::none()
            .inner_margin(Margin::same(2.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        for scene in self.scenes.iter() {
                            if ui.button(scene.title.as_str()).clicked() {
                                self.current_scene = Some(scene.clone());
                            }
                        }
                    });
                    ui.vertical(|ui| {
                        match &self.current_scene {
                            Some(elem) => ui.label(format!("Scene: {}", elem.title)),
                            None => ui.label("Please select a scene.")
                        };
                    });
                });
            });
    }

    fn title(&mut self) -> WidgetText {
        "Scene View".into()
    }
}