use eframe::egui;
use eframe::egui::{Align, Color32, FontId, Frame, TextFormat, Ui, Window};
use eframe::egui::style::Margin;
use eframe::egui::text::LayoutJob;
use egui_dock::Tab;

pub struct Editor {
    name: String,
    modified: bool,
    text: String,
    show_save: bool,
    exit: bool,
}

impl Editor {
    pub fn new(name: String) -> Self {
        Self {
            name,
            modified: false,
            text: "Important text to edit".into(),
            show_save: false,
            exit: false,
        }
    }

    fn save(&mut self) {
        self.modified = false;
        //save text to file or someplace else
    }
}

impl Tab for Editor {
    fn ui(&mut self, ui: &mut Ui) {
        if self.show_save {
            Window::new("Save")
                .collapsible(false)
                .show(ui.ctx(), |ui| {
                    ui.vertical(|ui| {
                        ui.label(format!(
                            "You have unsaved work on {} would you like to save",
                            self.name
                        ));
                        ui.horizontal(|ui| {
                            if ui.button("Save").clicked() {
                                self.save();
                                self.exit = true;
                                self.show_save = false;
                            }
                            if ui.button("Don't Save").clicked() {
                                self.exit = true;
                                self.show_save = false;
                            }
                            if ui.button("Cancel").clicked() {
                                self.exit = false;
                                self.show_save = false;
                            }
                        });
                    });
                });
        }
        Frame::none()
            .inner_margin(Margin::same(2.0))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            self.save();
                        }
                    });
                    if ui.code_editor(&mut self.text).changed() {
                        self.modified = true;
                    }
                });
            });
    }

    fn title(&mut self) -> egui::WidgetText {
        if self.modified {
            let mut job = LayoutJob::default();
            job.append(
                self.name.as_str(),
                0.0,
                TextFormat::simple(FontId::default(), Color32::from_rgb(245, 245, 67)),
            );

            job.append(
                " M",
                0.0,
                TextFormat {
                    font_id: FontId::proportional(FontId::default().size / 1.5),
                    color: Color32::from_rgb(245, 245, 67),
                    valign: Align::Min,
                    ..Default::default()
                },
            );

            job.into()
        } else {
            self.name.clone().into()
        }
    }

    fn on_close(&mut self) -> bool {
        self.show_save = true;
        self.exit || !self.modified
    }

    fn force_close(&mut self) -> bool {
        self.exit
    }
}