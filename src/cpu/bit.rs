use crate::cpu::cpu::Cpu;

pub fn bit0b(c: &mut Cpu) {
    c.fz(c.registers.b & 0x01, 0);
}
pub fn bit0c(c: &mut Cpu) {
    c.fz(c.registers.c & 0x01, 0);
}
pub fn bit0d(c: &mut Cpu) {
    c.fz(c.registers.d & 0x01, 0);
}
pub fn bit0e(c: &mut Cpu) {
    c.fz(c.registers.e & 0x01, 0);
}
pub fn bit0h(c: &mut Cpu) {
    c.fz(c.registers.h & 0x01, 0);
}
pub fn bit0l(c: &mut Cpu) {
    c.fz(c.registers.l & 0x01, 0);
}
pub fn bit0a(c: &mut Cpu) {
    c.fz(c.registers.a & 0x01, 0);
}
pub fn bit0m(c: &mut Cpu) {
    let addr = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    c.fz(c.memory.rb(addr) & 0x01, 0);
}
pub fn bit1b(c: &mut Cpu) {
    c.fz(c.registers.b & 0x02, 0);
}
pub fn bit1c(c: &mut Cpu) {
    c.fz(c.registers.c & 0x02, 0);
}
pub fn bit1d(c: &mut Cpu) {
    c.fz(c.registers.d & 0x02, 0);
}
pub fn bit1e(c: &mut Cpu) {
    c.fz(c.registers.e & 0x02, 0);
}
pub fn bit1h(c: &mut Cpu) {
    c.fz(c.registers.h & 0x02, 0);
}
pub fn bit1l(c: &mut Cpu) {
    c.fz(c.registers.l & 0x02, 0);
}
pub fn bit1a(c: &mut Cpu) {
    c.fz(c.registers.a & 0x02, 0);
}
pub fn bit1m(c: &mut Cpu) {
    let addr = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    c.fz(c.memory.rb(addr) & 0x02, 0);
}
pub fn bit2b(c: &mut Cpu) {
    c.fz(c.registers.b & 0x04, 0);
}
pub fn bit2c(c: &mut Cpu) {
    c.fz(c.registers.c & 0x04, 0);
}
pub fn bit2d(c: &mut Cpu) {
    c.fz(c.registers.d & 0x04, 0);
}
pub fn bit2e(c: &mut Cpu) {
    c.fz(c.registers.e & 0x04, 0);
}
pub fn bit2h(c: &mut Cpu) {
    c.fz(c.registers.h & 0x04, 0);
}
pub fn bit2l(c: &mut Cpu) {
    c.fz(c.registers.l & 0x04, 0);
}
pub fn bit2a(c: &mut Cpu) {
    c.fz(c.registers.a & 0x04, 0);
}
pub fn bit2m(c: &mut Cpu) {
    let addr = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    c.fz(c.memory.rb(addr) & 0x04, 0);
}
pub fn bit3b(c: &mut Cpu) {
    c.fz(c.registers.b & 0x08, 0);
}
pub fn bit3c(c: &mut Cpu) {
    c.fz(c.registers.c & 0x08, 0);
}
pub fn bit3d(c: &mut Cpu) {
    c.fz(c.registers.d & 0x08, 0);
}
pub fn bit3e(c: &mut Cpu) {
    c.fz(c.registers.e & 0x08, 0);
}
pub fn bit3h(c: &mut Cpu) {
    c.fz(c.registers.h & 0x08, 0);
}
pub fn bit3l(c: &mut Cpu) {
    c.fz(c.registers.l & 0x08, 0);
}
pub fn bit3a(c: &mut Cpu) {
    c.fz(c.registers.a & 0x08, 0);
}
pub fn bit3m(c: &mut Cpu) {
    let addr = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    c.fz(c.memory.rb(addr) & 0x08, 0);
}
pub fn bit4b(c: &mut Cpu) {
    c.fz(c.registers.b & 0x10, 0);
}
pub fn bit4c(c: &mut Cpu) {
    c.fz(c.registers.c & 0x10, 0);
}
pub fn bit4d(c: &mut Cpu) {
    c.fz(c.registers.d & 0x10, 0);
}
pub fn bit4e(c: &mut Cpu) {
    c.fz(c.registers.e & 0x10, 0);
}
pub fn bit4h(c: &mut Cpu) {
    c.fz(c.registers.h & 0x10, 0);
}
pub fn bit4l(c: &mut Cpu) {
    c.fz(c.registers.l & 0x10, 0);
}
pub fn bit4a(c: &mut Cpu) {
    c.fz(c.registers.a & 0x10, 0);
}
pub fn bit4m(c: &mut Cpu) {
    let addr = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    c.fz(c.memory.rb(addr) & 0x10, 0);
}
pub fn bit5b(c: &mut Cpu) {
    c.fz(c.registers.b & 0x20, 0);
}
pub fn bit5c(c: &mut Cpu) {
    c.fz(c.registers.c & 0x20, 0);
}
pub fn bit5d(c: &mut Cpu) {
    c.fz(c.registers.d & 0x20, 0);
}
pub fn bit5e(c: &mut Cpu) {
    c.fz(c.registers.e & 0x20, 0);
}
pub fn bit5h(c: &mut Cpu) {
    c.fz(c.registers.h & 0x20, 0);
}
pub fn bit5l(c: &mut Cpu) {
    c.fz(c.registers.l & 0x20, 0);
}
pub fn bit5a(c: &mut Cpu) {
    c.fz(c.registers.a & 0x20, 0);
}
pub fn bit5m(c: &mut Cpu) {
    let addr = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    c.fz(c.memory.rb(addr) & 0x20, 0);
}
pub fn bit6b(c: &mut Cpu) {
    c.fz(c.registers.b & 0x40, 0);
}
pub fn bit6c(c: &mut Cpu) {
    c.fz(c.registers.c & 0x40, 0);
}
pub fn bit6d(c: &mut Cpu) {
    c.fz(c.registers.d & 0x40, 0);
}
pub fn bit6e(c: &mut Cpu) {
    c.fz(c.registers.e & 0x40, 0);
}
pub fn bit6h(c: &mut Cpu) {
    c.fz(c.registers.h & 0x40, 0);
}
pub fn bit6l(c: &mut Cpu) {
    c.fz(c.registers.l & 0x40, 0);
}
pub fn bit6a(c: &mut Cpu) {
    c.fz(c.registers.a & 0x40, 0);
}
pub fn bit6m(c: &mut Cpu) {
    let addr = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    c.fz(c.memory.rb(addr) & 0x40, 0);
}
pub fn bit7b(c: &mut Cpu) {
    c.fz(c.registers.b & 0x80, 0);
}
pub fn bit7c(c: &mut Cpu) {
    c.fz(c.registers.c & 0x80, 0);
}
pub fn bit7d(c: &mut Cpu) {
    c.fz(c.registers.d & 0x80, 0);
}
pub fn bit7e(c: &mut Cpu) {
    c.fz(c.registers.e & 0x80, 0);
}
pub fn bit7h(c: &mut Cpu) {
    c.fz(c.registers.h & 0x80, 0);
}
pub fn bit7l(c: &mut Cpu) {
    c.fz(c.registers.l & 0x80, 0);
}
pub fn bit7a(c: &mut Cpu) {
    c.fz(c.registers.a & 0x80, 0);
}
pub fn bit7m(c: &mut Cpu) {
    let addr = ((c.registers.h as u16) << 8) + c.registers.l as u16;
    c.fz(c.memory.rb(addr) & 0x80, 0);
}