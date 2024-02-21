use std::{fs::File, io::{Read, Result}};

pub struct CPU {
    memory: RAM,
    running: bool,
    xreg: [u32; 32],
    pc: u32,
    dec_inst: Option<Instruction>,
}

impl CPU {
    pub fn new() -> Self {
        Self { memory: RAM::new(), running: true, xreg: [0; 32], pc: 0, dec_inst: None }
    }

    pub fn run(&mut self) {
        while self.running {
            self.fetch();
            self.decode();
            self.execute();
        }
    }

    fn fetch(&mut self) {
        todo!()
    }

    fn decode(&mut self) {
        todo!()
    }

    fn execute(&mut self) {
        todo!()
    }
}

pub struct RAM {
    mem: Vec<u8>,
}

impl RAM {
    pub fn new() -> Self {
        Self { mem: vec![0; 1073741824] }
    }

    pub fn from_raw(mut raw: File) -> Result<RAM> {
        let mut buffer: Vec<u8> = Vec::new();
        raw.read_to_end(&mut buffer)?;
        Ok(Self { mem: buffer })
    }
}

struct Instruction {
    opcode: Opcode,
    rs1: IOReg,
    rs2: IOReg,
    rd: IOReg,
}

enum Opcode {}

enum IOReg {
    Ptr(u32),
    Reg(u8),
    None,
}
