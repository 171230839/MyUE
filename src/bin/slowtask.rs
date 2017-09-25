


struct FSlowTask{
    pub defaultMessage: String,
    pub frameMessage: String,
    pub totalAmountOfWork: f32,
    pub completedWork: f32,
    pub currentFrameScope: f32,

    pub startTime: f64,
    pub openDialogThreshold : Option<f32>,

    bEnabled: bool,
    bCreateDialog: bool,
    
}


impl FSlowTask{
    pub fn new(inAmountOfWork: f32, inDefaultMessage : &str, bInEnabled: bool, ) -> Self{

    }
}