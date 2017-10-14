use logger::{ELogVerbosity, ELogTimes};
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;


pub fn FormatLogLine(category : &String, level: &ELogVerbosity, data: &String, time: &u64, logTime : ELogTimes) -> String{
    let mut result = String::new();
   
    match logTime{
            ELogTimes::SinceGStartTime => {
                  result = format!("[{:.*}]",10, time);      
            },
            ELogTimes::UTC =>{  
                let system_time = SystemTime::now();
                let datetime: DateTime<Utc> = system_time.into();
                result = format!("{}", datetime.format("%d/%m/%Y %T"));
            },
            NONE => (),
    }
  //println!("time: {:?}", time);
    result.push_str(&category);
    result.push_str(":");
    result.push_str(&level.to_string());
    result.push_str(": ");
    result.push_str(&data);

    
    #[cfg(target_os = "windows")]
        result.push_str("\r\n");

    result.clone()
}