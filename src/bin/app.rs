
#[derive(Default)]
pub struct FApp{
    gameName: String,
}

impl FApp{

    pub fn SetGameName(&mut self, name : &String){
        self.gameName = name.clone();
    }

    pub fn HasGameName(&self) -> bool{
        !self.gameName.is_empty()
    }

    pub fn GetGameName(&self) -> String{
        self.gameName.clone()
    }
}