use std::{fmt::Display, error::Error};

#[derive(Debug)]
pub enum CustomError {
    Absent,
    Full,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Absent => {write!(f, "Primitive absent.")?;}
            Self::Full => {write!(f, "Threadpool full.")?;}
        }
        Ok(())
    }
}

impl Error for CustomError {}