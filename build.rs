

use std::env;
pub fn main(){
    let key = "RUST_LOG";
    let mut env_log = String::new();
    match env::var_os(key){
        Some(val) => env_log.push_str(val.into_string().as_ref().unwrap()),
        None => {}
    }
    if env_log.is_empty(){
        env_log.push_str("info");
    }

   

  //  println!("cargo:rustc-env=RUST_LOG={}", env_log);
  //  println!("cargo:rustc-cfg=WITH_EDITOR");
   // println!("cargo:rustc-cfg=WITH_TEST");
}
