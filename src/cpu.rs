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

#[derive(Clone, Copy, Debug)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone, Copy, Debug)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

#[derive(Clone, Copy, Debug)]
pub struct Imm8;

#[derive(Clone, Copy, Debug)]
pub struct Imm16;

#[derive(Clone, Copy, Debug)]
pub enum Indirect {
    BC,
    DE,
    HL,
    CFF,
    HLD,
    HLI,
}

#[derive(Clone, Copy, Debug)]
pub enum Direct8 {
    D,
    DFF,
}

#[derive(Clone, Copy, Debug)]
pub enum Cond {
    NZ,
    Z,
    NC,
    C,
}

#[derive(Clone, Copy, Debug)]
pub struct Direct16;

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

impl IO8<Reg8> for Cpu {
    fn read8(&mut self, _: &Peripherals, src: Reg8) -> Option<u8> {
        Some(match src {
            Reg8::A => self.regs.a,
            Reg8::B => self.regs.b,
            Reg8::C => self.regs.c,
            Reg8::D => self.regs.d,
            Reg8::E => self.regs.e,
            Reg8::H => self.regs.h,
            Reg8::L => self.regs.l,
        })
    }

    fn write8(&mut self, _: &mut Peripherals, dst: Reg8, val: u8) -> Option<()> {
        Some(match dst {
            Reg8::A => self.regs.a = val,
            Reg8::B => self.regs.b = val,
            Reg8::C => self.regs.c = val,
            Reg8::D => self.regs.d = val,
            Reg8::E => self.regs.e = val,
            Reg8::H => self.regs.h = val,
            Reg8::L => self.regs.l = val,
        })
    }
}

impl IO16<Reg16> for Cpu {
    fn read16(&mut self, _: &Peripherals, src: Reg16) -> Option<u16> {
        Some(match src {
            Reg16::AF => self.regs.af(),
            Reg16::BC => self.regs.bc(),
            Reg16::DE => self.regs.de(),
            Reg16::HL => self.regs.hl(),
            Reg16::SP => self.regs.sp,
        })
    }

    fn write16(&mut self, _: &mut Peripherals, dst: Reg16, val: u16) -> Option<()> {
        Some(match dst {
            Reg16::AF => self.regs.write_af(val),
            Reg16::BC => self.regs.write_bc(val),
            Reg16::DE => self.regs.write_de(val),
            Reg16::HL => self.regs.write_de(val),
            Reg16::SP => self.regs.write_hl(val),
        })
    }
}
