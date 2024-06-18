mod operands;

use crate::peripherals;
use crate::peripherals::Peripherals;
use crate::registers;

pub trait IO8<T: Copy> {
    fn read8(&mut self, bus: &Peripherals, src: T) -> Option<u8>;
    fn write8(&mut self, bus: &mut Peripherals, dst: T, val: u8) -> Option<()>;
}

pub trait IO16<T: Copy> {
    fn read16(&mut self, bus: &Peripherals, src: T) -> Option<u16>;
    fn write16(&mut self, bus: &mut Peripherals, dst: T, val: u16) -> Option<()>;
}

#[derive(Default)]
struct Ctx {
    opcode: u8,
    cb: bool,
}

pub struct Cpu {
    regs: registers::Registers,
    ctx: Ctx,
}

impl Cpu {
    pub fn emulate_cycle(&mut self, bus: &mut Peripherals) {
        self.decode(bus);
    }

    pub fn fetch(&mut self, bus: &peripherals::Peripherals) {
        self.ctx.opcode = bus.read(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);
        self.ctx.cb = false;
    }

    pub fn decode(&mut self, bus: &mut Peripherals) {
        match self.ctx.opcode {
            0x00 => self.nop(bus),
            _ => panic!("Not implemented: {:02x}", self.ctx.opcode),
        }
    }

    pub fn nop(&mut self, bus: &Peripherals) {
        self.fetch(bus);
    }
}

impl IO8<operands::Reg8> for Cpu {
    fn read8(&mut self, _: &Peripherals, src: operands::Reg8) -> Option<u8> {
        Some(match src {
            operands::Reg8::A => self.regs.a,
            operands::Reg8::B => self.regs.b,
            operands::Reg8::C => self.regs.c,
            operands::Reg8::D => self.regs.d,
            operands::Reg8::E => self.regs.e,
            operands::Reg8::H => self.regs.h,
            operands::Reg8::L => self.regs.l,
        })
    }

    fn write8(&mut self, _: &mut Peripherals, dst: operands::Reg8, val: u8) -> Option<()> {
        Some(match dst {
            operands::Reg8::A => self.regs.a = val,
            operands::Reg8::B => self.regs.b = val,
            operands::Reg8::C => self.regs.c = val,
            operands::Reg8::D => self.regs.d = val,
            operands::Reg8::E => self.regs.e = val,
            operands::Reg8::H => self.regs.h = val,
            operands::Reg8::L => self.regs.l = val,
        })
    }
}

impl IO16<operands::Reg16> for Cpu {
    fn read16(&mut self, _: &Peripherals, src: operands::Reg16) -> Option<u16> {
        Some(match src {
            operands::Reg16::AF => self.regs.af(),
            operands::Reg16::BC => self.regs.bc(),
            operands::Reg16::DE => self.regs.de(),
            operands::Reg16::HL => self.regs.hl(),
            operands::Reg16::SP => self.regs.sp,
        })
    }

    fn write16(&mut self, _: &mut Peripherals, dst: operands::Reg16, val: u16) -> Option<()> {
        Some(match dst {
            operands::Reg16::AF => self.regs.write_af(val),
            operands::Reg16::BC => self.regs.write_bc(val),
            operands::Reg16::DE => self.regs.write_de(val),
            operands::Reg16::HL => self.regs.write_de(val),
            operands::Reg16::SP => self.regs.write_hl(val),
        })
    }
}
