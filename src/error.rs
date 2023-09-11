use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum SocketryError {
    Absent,
    Full,
}

impl Display for SocketryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Absent => {
                write!(f, "Primitive absent.")?;
            }
            Self::Full => {
                write!(f, "Threadpool full.")?;
            }
        }
        Ok(())
    }
}

impl Error for SocketryError {}
