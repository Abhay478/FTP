use std::fs;

use crate::{Job, Res};

pub enum Primitives {
    FileShare,
}

impl Primitives {
    pub fn get_job(&self) -> Res<Job> {
        match self {
            Self::FileShare => Ok(Box::new(ftp)),
        }
    }
}

fn ftp(v: Vec<u8>) -> Res<Vec<u8>> {
    let path = v.iter().map(|u| *u as char).collect::<String>();
    println!("FTP request for {path} received.");
    match fs::read(path) {
        Ok(x) => Ok(x),
        Err(e) => Ok(e.to_string().as_bytes().to_vec()),
    }
}
