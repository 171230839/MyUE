

use std::ffi::OsStr;
pub struct FProjectDesriptor{

}

impl FProjectDesriptor{
    pub fn GetExtension() ->String{
        String::from("uproject").clone()
    }
}