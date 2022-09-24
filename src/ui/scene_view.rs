use std::fmt::format;
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
    pub fn new(scenes: Vec<Rc<Scene>>) -> Self {
        let first_scene;
        if scenes.len() > 0 {
            first_scene = Some(scenes[0].clone());
        } else {
            first_scene = None;
        }

        Self {
            scenes,
            current_scene: first_scene,
        }
    }
}

impl Tab for SimpleSceneView {
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.draw_scenes_ui(ui);
            ui.allocate_space(ui.available_size());
        });
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Add Scene").clicked() {
                let index = i32::try_from(self.scenes.len());
                let mut scene_index = i32::MAX;
                match index {
                    Ok(val) => scene_index = val + 1,
                    Err(_) => ()
                }
                self.scenes.push(Rc::new(Scene { index: scene_index, title: format!("Scene {}", self.scenes.len() + 1) }));
            }
        });
    }

    fn title(&mut self) -> WidgetText {
        "Scene View".into()
    }
}

impl SimpleSceneView {
    fn draw_scenes_ui(&mut self, ui: &mut Ui) {
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
    }
}