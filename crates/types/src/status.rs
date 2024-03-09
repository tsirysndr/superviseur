use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum Status {
    Running,
    Stopped,
    Unknown,
}

impl Default for Status {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Running => write!(f, "running"),
            Self::Stopped => write!(f, "stopped"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "running" => Ok(Self::Running),
            "stopped" => Ok(Self::Stopped),
            _ => Err(format!("Unknown status: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_from_str() {
        assert_eq!(Status::from_str("running").unwrap(), Status::Running);
        assert_eq!(Status::from_str("stopped").unwrap(), Status::Stopped);
        assert_eq!(
            Status::from_str("unknown"),
            Err("Unknown status: unknown".to_string())
        );
        assert!(Status::from_str("invalid").is_err());
    }
}
