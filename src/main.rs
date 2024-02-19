use std::{error::Error, io::Read, vec::Vec};

struct CPU {
    memory: Box<RAM>,
    running: bool,
}

impl CPU {
    fn new() -> Self {
        Self { memory: Box::new(RAM::new()), running: true }
    }

    fn run(&mut self) {
        while self.running {
            self.fetch();
            self.decode();
            self.execute();
        }
    }

    fn fetch(&mut self) {
        todo!();
    }

    fn decode(&mut self) {
        todo!();
    }

    fn execute(&mut self) {
        todo!();
    }
}

struct RAM {
    mem: Vec<u8>,
}

impl RAM {
    fn new() -> Self {
        Self { mem: Vec::new() }
    }

    fn new_from_raw(mut raw: std::fs::File) -> std::io::Result<RAM> {
        let mut buffer: Vec<u8> = Vec::new();

        raw.read_to_end(&mut buffer)?;

        Ok(RAM { mem: buffer })
    }
}

fn main() {
    let CPU = CPU::new();
}
