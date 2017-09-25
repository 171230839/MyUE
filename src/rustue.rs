
use std::fmt;

pub struct FEngineVersion {
    pub major: String,
    pub minor: String,
    pub patch: String,
}
impl fmt::Display for FEngineVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cargo {}.{}.{}", self.major, self.minor, self.patch)?;
        Ok(())
    }
}
pub fn version() -> FEngineVersion {
    macro_rules! env_str {
        ($name:expr) => { env!($name).to_string() }
    }
    FEngineVersion {
        major: env_str!("CARGO_PKG_VERSION_MAJOR"),
        minor: env_str!("CARGO_PKG_VERSION_MINOR"),
        patch: env_str!("CARGO_PKG_VERSION_PATCH"),
    }
}
