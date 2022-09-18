use eframe::egui::{Context, Ui, SidePanel, CentralPanel, Area, pos2};

#[derive(Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FileBrowser {
    pub packs: Vec<String>,
}

impl FileBrowser {
    pub fn draw(self, ctx: &Context, ui: &mut Ui) {

        Area::new("my_area")
                    //.default_pos(pos2(32.0, 32.0))
                    .drag_bounds(eframe::epaint::Rect { min: pos2(200.0, 40.0), max: pos2(400.0, 80.0) })
                    .interactable(true)
                    .show(ctx, |ui| {
                        ui.label("Floating text!");
                    });
        /* 
        SidePanel::left("content_browser").show(ctx, |ui_folders|{
            ui_folders.label("folder 1");
            ui_folders.label("folder 2");
        });

        CentralPanel::default().show(ctx, |ui_files|{
            ui_files.label("file1");
            ui_files.label("file 2");
        });
        */
    }
}