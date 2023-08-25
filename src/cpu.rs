use crate::instruction::Instruction;
use crate::stack::Stack;
use log::{debug, error, info};
use rand::random;
use std::fs::File;
use std::io::{stdout, Read, Write};
use std::time;
use std::time::Duration;

const PROGRAM_LOAD_MEMORY_OFFSET: i64 = 0x200;

#[derive(Debug)]
pub struct Chip8 {
    memory: [u8; 4096],
    stack: Stack<usize>,
    pc: usize,
    v: Vec<u8>,
    i: u16,
    dt: u16,
    st: u16,
}

impl Chip8 {
    fn tick(&mut self) {
        self.pc += 2;

        if self.dt > 0 {
            self.dt -= 1;
        }

        let raw_inst: u16 = u16::from_be_bytes([self.memory[self.pc], self.memory[self.pc + 1]]);
        let instruction = Instruction::from(raw_inst);
        println!("instruction: {:?}", instruction);
        match instruction {
            Instruction::RET => {
                self.pc = match self.stack.pop() {
                    Some(pc) => pc,
                    None => {
                        error!("tried to return but stack is empty, bailing");
                        panic!();
                    }
                };
            }
            Instruction::CALL(addr) => match self.stack.push(self.pc) {
                Ok(_) => self.pc = addr,
                Err(e) => {
                    error!("stack overflow! bailing");
                    panic!();
                }
            },
            Instruction::SE_VX_KK(x, byte) => {
                if self.v[x] == byte {
                    self.pc += 2;
                }
            }
            Instruction::SNE_VX_KK(x, byte) => {
                if self.v[x] != byte {
                    self.pc += 2;
                }
            }
            Instruction::SE_VX_VY(x, y) => {
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }
            Instruction::LD_VX_K(x, byte) => {
                self.v[x] = byte as u8;
            }
            Instruction::ADD_VX_KK(x, byte) => {
                self.v[x] = byte as u8;
            }
            Instruction::LD_VX_VY(x, y) => {
                self.v[x] = y as u8;
            }
            Instruction::OR_VX_VY(x, y) => {
                self.v[x] = x as u8 | y as u8;
            }
            Instruction::AND_VX_VY(x, y) => {
                // possible fixme: Documentation says to check if both bits are one
                self.v[x] = x as u8 & y as u8;
            }
            Instruction::XOR_VX_VY(x, y) => {
                self.v[x] = x as u8 ^ y as u8;
            }
            Instruction::ADD_VX_VY(x, y) => {
                // fixme: make sure casting is really the bests way to do this
                let result = self.v[y] as u16 + self.v[x] as u16;
                if result > 255 {
                    self.v[0xF - 1] = 1;
                    self.v[x] = result as u8;
                } else {
                    self.v[0xF - 1] = 0;
                }
            }
            Instruction::SUB_VX_VY(x, y) => {
                if self.v[x] > self.v[y] {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] = self.v[y] - self.v[x];
            }
            Instruction::SHR_VX_VY(x, y) => {
                self.v[x] >>= 1;
                if self.v[x] & 0b1 == 1 {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] /= 2;
            }
            Instruction::SUBN_VX_VY(x, y) => {
                // self.v[x] = self.v[y] - self.v[x];
                // self.v[0xF] = !self.v[0xF];
                if self.v[y] > self.v[x] {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] = self.v[y] - self.v[x];
            }
            Instruction::SHL_VX_VY(x, y) => {
                self.v[x] <<= 1;
                if self.v[x] & 0b1 == 1 {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] *= 2;
            }
            Instruction::SNE_VX_VY(x, y) => {
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }
            Instruction::LD_I_ADDR(addr) => {
                self.i = addr as u16;
            }
            Instruction::JP_V0_ADDR(addr) => {
                self.pc = addr + self.v[0x0] as usize;
            }
            Instruction::RND_VX_KK(x, byte) => {
                self.v[x] = random::<u8>() & byte;
            }
            Instruction::DRW_VX_VY_NIB(x, y, size) => {}
            Instruction::LD_VX_DT(x) => {
                self.v[x] = self.dt as u8;
            }
            Instruction::LD_DT_VX(x) => {
                self.dt = x as u16;
            }
            Instruction::LD_ST_VX(x) => {
                self.st = self.v[x] as u16;
            }
            Instruction::ADD_I_VX(x) => {
                self.i += self.v[x] as u16;
            }
            Instruction::LD_I_VX(address) => {
                for (i, val) in self.v.iter().enumerate() {
                    self.memory[address + i] = *val;
                }
            }
            Instruction::LD_VX_I(x) => {
                for (i, val) in self.memory[x..x + 0xF].iter().enumerate() {
                    self.v[i] = *val;
                }
            }
            Instruction::LD_B_VX(byte, x) => {
                self.v[x] = byte;
            }
            Instruction::LD_BCD_VX(b, _) => {
                self.memory[self.i as usize] = (b / 100) % 100;
                self.memory[(self.i + 1) as usize] = (b / 10) % 10;
                self.memory[(self.i + 2) as usize] = b % 10;
            }
            Instruction::JP(addr) => {
                self.pc = addr;
            }
            _ => {
                info!(
                    "attempted invalid or unsupported instruction: {:?}, ignored",
                    instruction
                )
            }
        }
    }

    pub fn dump_ram(&self) {
        let timestamp = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap(); // this should never panic. if it does, something is horribly wrong.
        let mut file = File::create(format!("dumps/{}.bin", timestamp.as_secs())).unwrap();
        file.write_all(&self.memory).unwrap();
    }

    pub fn from_file(path: &String) -> Result<Chip8, std::io::Error> {
        let mut cpu = Chip8 {
            memory: [0; 4096],
            stack: Stack::with_capacity(16),
            pc: 0,
            v: vec![0; 16],
            i: 0,
            dt: 0,
            st: 0,
        };
        let mut file = File::open(path)?;
        // TODO: todo: find clean way to handle possible error
        file.read(&mut cpu.memory)?;
        Ok(cpu)
    }

    pub fn run(&mut self) {
        loop {
            debug!(
                "stack={} pc={} v={:?} i={} dt={} st={}\r",
                self.stack, self.pc, self.v, self.i, self.dt, self.st
            );
            self.tick();
            print!(
                "stack={} pc={} v={:?} i={} dt={} st={}\r",
                self.stack, self.pc, self.v, self.i, self.dt, self.st
            );
            let _ = stdout().flush();
            std::thread::sleep(Duration::from_secs(1 / 10));
        }
    }
}
