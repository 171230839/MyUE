


pub fn main(){
    println!("ue -client");

    if cfg!(target = "client"){
        println!("target = client");
    }

}