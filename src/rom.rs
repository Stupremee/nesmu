use std::io::{self, prelude::*, BufReader};
use std::fs::File;

#[derive(Debug)]
pub struct Rom {
    data: Vec<u8>,
}

impl Rom {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn from_file(path: String) -> io::Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(Self { data: buf })
    }

    pub fn read(&self, addr: usize) -> Option<&u8> {
        self.data.get(addr)
    }
}
