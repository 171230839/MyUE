use path::FPaths;
use std::sync::{Mutex};

lazy_static!{
    pub static ref GIProjectManager: Mutex<IProjectManager> = Mutex::new(Default::default());
}



#[derive(Default)]
pub struct IProjectManager{
    recentProjectFileName: String,
}

impl IProjectManager{

    pub fn GetAutoLoadProjectFileName(&mut self) -> String{
        if self.recentProjectFileName.is_empty(){
            self.recentProjectFileName.push_str(&FPaths::GameAgnosticSavedDir());
            self.recentProjectFileName.push_str("AutoLoadProject.txt");
        }
        self.recentProjectFileName.clone()
    }
}