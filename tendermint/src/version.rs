use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::str::FromStr;

/// Tendermint version
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Version(String);

impl FromStr for Version {
    type Err = ();
    fn from_str(s: &str) -> Result<Version, ()> {
        Ok(Version(s.to_owned()))
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
