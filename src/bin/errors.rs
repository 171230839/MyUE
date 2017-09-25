use std::fmt;
use std::error::Error;

error_chain! {
    types {
        RustUEError, RustUEErrorKind,  RustUEResultExt,  RustUEResult;
    }
}


impl RustUEError {
    fn is_human(&self) -> bool {
        match &self.0 {
            &RustUEErrorKind::Msg(_) => true,
            _ => false,
        }
    }
}

pub type RueResult = Result<(), RueError>;

#[derive(Debug)]
pub struct RueError {
    pub error: Option<RustUEError>,
    pub unknown: bool,
    pub exit_code: i32,
}

impl Error for RueError {
    fn description(&self) -> &str {
        self.error
            .as_ref()
            .map(|e| e.description())
            .unwrap_or("unknown Rue error")
    }

    fn cause(&self) -> Option<&Error> {
        self.error.as_ref().and_then(|e| e.cause())
    }
}

impl fmt::Display for RueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref error) = self.error {
            error.fmt(f)
        } else {
            self.description().fmt(f)
        }
    }
}

impl RueError {
    pub fn new(error: RustUEError, code: i32) -> RueError {
        let human = &error.is_human();
        RueError {
            error: Some(error),
            exit_code: code,
            unknown: !human,
        }
    }

    pub fn code(code: i32) -> RueError {
        RueError {
            error: None,
            exit_code: code,
            unknown: false,
        }
    }
}

impl From<RustUEError> for RueError {
    fn from(err: RustUEError) -> RueError {
        RueError::new(err, 101)
    }
}
