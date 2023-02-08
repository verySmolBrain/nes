use crate::emulator::cpu::Cpu;
use crate::emulator::memory::Mem;

#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    Relative,
    NoneAddressing,
    Accumulator,
    Implied,
    JumpIndirect,
    Jump,
}

impl Cpu {
    pub fn get_operand_address(&mut self, mode: &AddressingMode) -> (Option<u16>, u16, bool) {
        match mode {
            AddressingMode::Immediate => {
                let addr = self.program_counter;
                (Some(addr), 1, false)
            },
            AddressingMode::ZeroPage => {
                let addr = self.mem_read(self.program_counter) as u16;
                (Some(addr), 1, false)
            },
            AddressingMode::Absolute | AddressingMode::Jump => {
                let addr = self.mem_read_u16(self.program_counter);
                (Some(addr), 2, false)
            },
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x);
                (Some(addr as u16), 1, false)
            },
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y);
                (Some(addr as u16), 1, false)
            },
            AddressingMode::Absolute_X => {
                let pos = self.mem_read_u16(self.program_counter);
                let addr = pos.wrapping_add(self.register_x as u16);
                let crossed_page = (pos & 0xFF00) != (addr & 0xFF00);
                (Some(addr), 2, crossed_page)
            },
            AddressingMode::Absolute_Y => {
                let pos = self.mem_read_u16(self.program_counter);
                let addr = pos.wrapping_add(self.register_y as u16);
                let crossed_page = (pos & 0xFF00) != (addr & 0xFF00);
                (Some(addr), 2, crossed_page)
            },
            AddressingMode::Indirect_X => {
                let pos = self.mem_read(self.program_counter);
                let ptr = pos.wrapping_add(self.register_x);

                let addr = u16::from_le_bytes([ // Indexed Indirect adding before lookup
                    self.mem_read(ptr as u16),
                    self.mem_read(ptr.wrapping_add(1) as u16)
                ]);
                (Some(addr), 1, false)
            },
            AddressingMode::Indirect_Y => {
                let pos = self.mem_read(self.program_counter);
                let ptr = u16::from_le_bytes([
                    self.mem_read(pos as u16),
                    self.mem_read(pos.wrapping_add(1) as u16)
                ]); // Indirect Index adding after lookup

                let addr = ptr.wrapping_add(self.register_y as u16);
                let crossed_page = (ptr & 0xFF00) != (addr & 0xFF00);

                (Some(addr), 1, crossed_page)
            },
            AddressingMode::Relative => {
                let relative = self.mem_read(self.program_counter) as i8;
                let addr = self.program_counter
                    .wrapping_add(relative as u16)
                    .wrapping_add(1);
                let crossed_page = (self.program_counter & 0xFF00) != (addr & 0xFF00);
                (Some(addr), 1, crossed_page)
            },
            AddressingMode::Accumulator => {
                (None, 0, false)
            },
            AddressingMode::Implied => {
                (None, 0, false)
            },
            /*
            An original 6502 has does not correctly fetch the target address if the 
            indirect vector falls on a page boundary (e.g. $xxFF where xx is any value from $00 to $FF). 
            In this case fetches the LSB from $xxFF as expected but takes the MSB from $xx00.
            https://www.nesdev.org/obelisk-6502-guide/reference.html#JMP
            */
            AddressingMode::JumpIndirect => {
                let pos = self.mem_read_u16(self.program_counter);
                let addr = if pos & 0x00FF == 0x00FF {
                    u16::from_le_bytes([
                        self.mem_read(pos),
                        self.mem_read(pos & 0xFF00)
                    ])
                } else {
                    self.mem_read_u16(pos)
                };

                (Some(addr), 2, false)
            },
            AddressingMode::NoneAddressing => {
                (None, 0, false)
            }
        }
    }
}