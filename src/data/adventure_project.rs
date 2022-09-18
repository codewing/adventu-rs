use std::rc::Rc;
use crate::data::scene::Scene;

pub struct AdventureProject {
    pub title: String,
    pub scenes: Vec<Rc<Scene>>
}

impl AdventureProject {
    pub fn new(title: String) -> Self {
        Self {
            title,
            scenes: vec![
                Rc::new(Scene { index: 1, title: String::from("Scene 1") }),
                Rc::new(Scene { index: 2, title: String::from("Scene 2") }),
            ]
        }
    }

    pub fn rename_project(&mut self, new_title: String) {
        self.title = new_title;
    }
}