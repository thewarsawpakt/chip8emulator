// use twelve_bit::u12::{*, self};
use log::debug;
use twelve_bit::u12::U12;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Instruction {
    INVALID(u16),
    SYS(usize),
    CLS,
    RET,
    JP(usize),
    CALL(usize),
    SE_VX_KK(usize, u8),
    SNE_VX_KK(usize, u8),
    SE_VX_VY(usize, usize),
    LD_X_KK(usize, u8),
    ADD_VX_KK(usize, u8),
    LD_VX_VY(usize, usize),
    OR_VX_VY(usize, usize),
    AND_VX_VY(usize, usize),
    XOR_VX_VY(usize, usize),
    ADD_VX_VY(usize, usize),
    SUB_VX_VY(usize, usize),
    SHR_VX_VY(usize, usize),
    SUBN_VX_VY(usize, usize),
    SHL_VX_VY(usize, usize),
    SNE_VX_VY(usize, usize),
    LD_I_ADDR(usize),
    JP_V0_ADDR(usize),
    RND_VX_KK(usize, u8),
    DRW_VX_VY_NIB(usize, usize, U12),
    SKP_VX(usize),
    SKNP_VX(usize),
    LD_VX_DT(usize),
    LD_VX_K(usize, u8),
    LD_DT_VX(usize),
    LD_ST_VX(usize),
    ADD_I_VX(usize),
    LD_F_VX(usize),
    LD_BCD_VX(u8, usize),
    LD_VX_I(usize),
    LD_I_VX(usize),
    LD_B_VX(u8, usize),
}

impl From<u16> for Instruction {
    fn from(inst: u16) -> Self {
        /*
        From https://github.com/shlomnissan/chip8/blob/main/core/types.h
        */
        let high = inst >> 12;
        let low = inst & 0x000F;
        let x = (inst & 0x0F00 >> 8) as usize;
        let y = (inst & 0x00F0 >> 4) as usize;
        let byte = (inst & 0x00FF) as u8;
        let address = (inst & 0x0FFF) as usize;
        debug!(
            "raw={} high={} low={} vx={} vy={} byte={} address={}",
            inst, high, low, x, y, byte, address
        );
        match high {
            0x00 => match byte {
                0xE0 => Instruction::CLS,
                0xEE => Instruction::RET,
                _ => Instruction::INVALID(inst),
            },
            0x01 => Instruction::JP(address),
            0x02 => Instruction::CALL(address),
            0x03 => Instruction::SE_VX_KK(x, byte),
            0x04 => Instruction::SNE_VX_KK(x, byte),
            0x05 => Instruction::SE_VX_VY(x, y),
            0x06 => Instruction::LD_B_VX(byte, x),
            0x07 => Instruction::ADD_VX_KK(x, byte),
            0x08 => match low {
                0x00 => Instruction::LD_VX_VY(x, y),
                0x01 => Instruction::OR_VX_VY(x, y),
                0x02 => Instruction::AND_VX_VY(x, y),
                0x03 => Instruction::XOR_VX_VY(x, y),
                0x04 => Instruction::ADD_VX_VY(x, y),
                0x05 => Instruction::SUB_VX_VY(x, y),
                0x06 => Instruction::SHR_VX_VY(x, y),
                0x07 => Instruction::SUBN_VX_VY(x, y),
                0x0E => Instruction::SHL_VX_VY(x, y),
                _ => Instruction::INVALID(inst),
            },
            0x09 => Instruction::SNE_VX_VY(x, y),
            0x0A => Instruction::LD_I_ADDR(address),
            0x0C => Instruction::RND_VX_KK(x, byte),
            0x0D => Instruction::DRW_VX_VY_NIB(x, y, U12::from(byte)), // TODO: figure out which field the u12 should be read from
            0x0E => match low {
                0x9E => Instruction::SKP_VX(x),
                0xA1 => Instruction::SKNP_VX(x),
                _ => Instruction::INVALID(inst),
            },
            0x0F => match low {
                0x07 => Instruction::LD_VX_DT(x),
                0x0A => Instruction::LD_VX_K(x, byte),
                0x15 => Instruction::LD_DT_VX(x),
                0x18 => Instruction::LD_ST_VX(x),
                0x1E => Instruction::ADD_I_VX(x),
                0x29 => Instruction::LD_F_VX(x),
                0x33 => Instruction::LD_BCD_VX(byte, x),
                0x55 => Instruction::LD_I_VX(x),
                0x65 => Instruction::LD_VX_I(x),
                _ => Instruction::INVALID(inst),
            },
            _ => Instruction::INVALID(inst),
        }
    }
}
