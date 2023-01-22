use crate::{opcodes::OPCODES, bus::Bus, memory::Mem, rom::Rom };
use bitflags::bitflags;

/*  _______________ $10000  _______________
   | PRG-ROM       |       |               |
   | Upper Bank    |       |               |
   |_ _ _ _ _ _ _ _| $C000 | PRG-ROM       |
   | PRG-ROM       |       |               |
   | Lower Bank    |       |               |
   |_______________| $8000 |_______________|
   | SRAM          |       | SRAM          |
   |_______________| $6000 |_______________|
   | Expansion ROM |       | Expansion ROM |
   |_______________| $4020 |_______________|
   | I/O Registers |       |               |
   |_ _ _ _ _ _ _ _| $4000 |               |
   | Mirrors       |       | I/O Registers |
   | $2000-$2007   |       |               |
   |_ _ _ _ _ _ _ _| $2008 |               |
   | I/O Registers |       |               |
   |_______________| $2000 |_______________|
   | Mirrors       |       |               |
   | $0000-$07FF   |       |               |
   |_ _ _ _ _ _ _ _| $0800 |               |
   | RAM           |       | RAM           |
   |_ _ _ _ _ _ _ _| $0200 |               |
   | Stack         |       |               |
   |_ _ _ _ _ _ _ _| $0100 |               |
   | Zero Page     |       |               |
   |_______________| $0000 |_______________|
*/

pub const ROM_START: usize = 0x8600;
const RESET_VECTOR: usize = 0xFFFC;

const STACK: u16 = 0x0100; 
const STACK_RESET: u8 = 0xfd; // Push = store first then decrement. So 8 bit off for initial.


#[derive(Debug)]
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
}

/*
7  bit  0
---- ----
NVss DIZC
|||| ||||
|||| |||+- Carry
|||| ||+-- Zero
|||| |+--- Interrupt Disable
|||| +---- Decimal
||++------ No Cpu effect, see: the B flag
|+-------- Overflow
+--------- Negative
 */

bitflags! {
    pub struct Status: u8 {
        const NEGATIVE = 0b1000_0000;
        const OVERFLOW = 0b0100_0000;
        const BREAKONE = 0b0010_0000;
        const BREAKTWO = 0b0001_0000;
        const DECIMAL  = 0b0000_1000;
        const INTERDIS = 0b0000_0100;
        const ZERO     = 0b0000_0010;
        const CARRY    = 0b0000_0001;
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::BREAKONE | Status::INTERDIS
    }
}

pub struct Cpu {
    pub register_a: u8, 
    pub register_x: u8,
    pub register_y: u8,
    pub stack_pointer: u8,
    pub status: Status,
    pub program_counter: u16,
    pub bus: Bus
}

impl Cpu {
    pub fn new(rom: Rom) -> Self {
        let bus = Bus::new(rom);

        Cpu {
            register_a: 0, // accumulator
            register_x: 0,
            register_y: 0,
            stack_pointer: STACK_RESET,
            status: Default::default(), 
            program_counter: 0,
            bus
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = STACK_RESET;
        self.status = Default::default();

        self.program_counter = self.mem_read_u16(RESET_VECTOR as u16);
    }

    pub fn stack_push_u16(&mut self, value: u16) {
        value.to_le_bytes().iter().for_each(|v| {
            self.stack_push_u8(*v)
        })
    }

    pub fn stack_pop_u16(&mut self) -> u16 {
        u16::from_be_bytes([ // Since we push in LE, we need to pop in BE
            self.stack_pop_u8(),
            self.stack_pop_u8(),
        ])
    }

    pub fn stack_pop_u8(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.mem_read(STACK + self.stack_pointer as u16)
    }

    pub fn stack_push_u8(&mut self, value: u8) {
        self.mem_write(STACK + self.stack_pointer as u16, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> Option<u16> {
        match mode {
            AddressingMode::Immediate => {
                let addr = self.program_counter;
                self.program_counter = self.program_counter.wrapping_add(1);
                Some(addr)
            },
            AddressingMode::ZeroPage => {
                let addr = self.mem_read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                Some(addr)
            },
            AddressingMode::Absolute => {
                let addr = self.mem_read_u16(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(2);
                Some(addr)
            },
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x);
                self.program_counter = self.program_counter.wrapping_add(1);
                Some(addr as u16)
            },
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y);
                self.program_counter = self.program_counter.wrapping_add(1);
                Some(addr as u16)
            },
            AddressingMode::Absolute_X => {
                let pos = self.mem_read_u16(self.program_counter);
                let addr = pos.wrapping_add(self.register_x as u16);
                self.program_counter = self.program_counter.wrapping_add(2);
                Some(addr)
            },
            AddressingMode::Absolute_Y => {
                let pos = self.mem_read_u16(self.program_counter);
                let addr = pos.wrapping_add(self.register_y as u16);
                self.program_counter = self.program_counter.wrapping_add(2);
                Some(addr)
            },
            AddressingMode::Indirect_X => {
                let pos = self.mem_read(self.program_counter);
                let ptr = pos.wrapping_add(self.register_x);

                let addr = u16::from_le_bytes([ // Indexed Indirect adding before lookup
                    self.mem_read(ptr as u16),
                    self.mem_read(ptr.wrapping_add(1) as u16)
                ]);
                self.program_counter = self.program_counter.wrapping_add(1);
                Some(addr)
            },
            AddressingMode::Indirect_Y => {
                let pos = self.mem_read(self.program_counter);
                let ptr = u16::from_le_bytes([
                    self.mem_read(pos as u16),
                    self.mem_read(pos.wrapping_add(1) as u16)
                ]); // Indirect Index adding after lookup

                let addr = ptr.wrapping_add(self.register_y as u16);
                self.program_counter = self.program_counter.wrapping_add(1);
                Some(addr)
            },
            AddressingMode::Relative => {
                let relative = self.mem_read(self.program_counter) as i8;
                let addr = self.program_counter
                    .wrapping_add(relative as u16)
                    .wrapping_add(1);
                println!("Relative: {:x} -> {:x}", self.program_counter, addr);
                self.program_counter = self.program_counter.wrapping_add(1);
                Some(addr)
            },
            AddressingMode::Accumulator => {
                None
            },
            AddressingMode::Implied => {
                None
            },
            AddressingMode::NoneAddressing => {
                None
            }
        }
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        for i in 0..(program.len() as u16) {
            self.mem_write(((ROM_START as u16) + i) as u16, program[i as usize]);
        }
        self.mem_write_u16(RESET_VECTOR as u16, ROM_START as u16)
    }

    pub fn load_cartridge(&mut self, program: Vec<u8>) -> Result<(), String> {
        let cartridge = Rom::new(program)?;
        let new_bus = Bus::new(cartridge);

        self.bus = new_bus;
        self.reset();

        Ok(())
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut Cpu),
    {
        loop {
            let code = self.next();
            let opcode = OPCODES.get(&code).expect("Invalid opcode");
            let addr = self.get_operand_address(&opcode.mode);

            println!("PC: {:x} Code: {:x} Addr: {:x?} Mode: {:?}", self.program_counter, code, addr, opcode.mode);

            match code {
                0xA9 | 0xa5 | 0xb5 | 0xad | 0xbd |0xb9 | 0xa1 | 0xb1 => { /* LDA */
                    self.lda(addr.unwrap());
                },
                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => { /* STA */
                    self.sta(addr.unwrap())
                },
                0x84 | 0x94 | 0x8c => { /* STY */
                    self.sty(addr.unwrap())
                },
                0x86 | 0x96 | 0x8e => { /* STX */
                    self.stx(addr.unwrap())
                },
                0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 |0xc1 |0xd1 => { /* CMP */
                    self.cmp(addr.unwrap())
                },
                0xe0 | 0xe4 | 0xec => { /* CPX */
                    self.cpx(addr.unwrap())
                },
                0xc0 | 0xc4 | 0xcc => { /* CPY */
                    self.cpy(addr.unwrap()    )
                },
                0xc6 | 0xd6 | 0xce | 0xde => { /* DEC */
                    self.dec(addr.unwrap());
                },
                0xe6 | 0xf6 | 0xee | 0xfe => { /* INC */
                    self.inc(addr.unwrap())
                },
                0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe => { /* LDX */
                    self.ldx(addr.unwrap())
                },
                0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc => { /* LDY */
                    self.ldy(addr.unwrap())
                },
                0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => { /* AND */
                    self.and(addr.unwrap())
                },
                0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11 => { /* ORA */
                    self.ora(addr.unwrap())
                },
                0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => { /* EOR */
                    self.eor(addr.unwrap())
                },
                0x4c => { /* JMP Absolute */
                    self.jmp(addr.unwrap())
                }, 
                0x24 | 0x2c => { /* BIT */
                    self.bit(addr.unwrap())
                },
                0x66 | 0x76 | 0x6e | 0x7e => { /* ROR */
                    self.ror_mem(addr.unwrap())
                },
                0x26 | 0x36 | 0x2e | 0x3e => { /* ROL */
                    self.rol_mem(addr.unwrap())
                },
                0x06 | 0x16 | 0x0e | 0x1e => { /* ASL */
                    self.asl_mem(addr.unwrap())
                },
                0x46 | 0x56 | 0x4e | 0x5e => { /* LSR */
                    self.lsr_mem(addr.unwrap())
                },
                0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => { /* ADC */
                    self.adc(addr.unwrap())
                },
                0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => { /* SBC */
                    self.sbc(addr.unwrap())
                },
                0x20 => {
                    self.jsr(addr.unwrap())
                }, /* JSR */
                0x6a => self.ror_acc(), /* ROR Accumulator */
                0x2a => self.rol_acc(), /* ROL Accumulator */
                0x0a => self.asl_acc(), /* ASL Accumulator */
                0x4a => self.lsr_acc(), /* LSR Accumulator */
                0x6c => self.jmp_ind(), /* JMP Indirect */
                0x48 => self.pha(), /* PHA */
                0x08 => self.php(), /* PHP */
                0x68 => self.pla(), /* PLA */
                0x28 => self.plp(), /* PLP */
                0xc8 => self.iny(), /* INY */
                0xca => self.dex(), /* DEX */
                0x88 => self.dey(), /* DEY */
                0xa8 => self.tay(), /* TAY */
                0xba => self.tsx(), /* TSX */
                0x8a => self.txa(), /* TXA */
                0x9a => self.txs(), /* TXS */
                0x98 => self.tya(), /* TYA */
                0x38 => self.sec(), /* SEC */
                0xf8 => self.sed(), /* SED */
                0x78 => self.sei(), /* SEI */
                0xE8 => self.inx(), /* INX */
                0xAA => self.tax(), /* TAX */
                0x18 => self.clc(), /* CLC */
                0xd8 => self.cld(), /* CLD */
                0x58 => self.cli(), /* CLI */
                0xb8 => self.clv(), /* CLV */
                0x60 => self.rts(), /* RTS */
                0x40 => self.rti(), /* RTI */
                0xea => (), /* NOP */
                0x00 => return, /* BRK */
                0xb0 => if self.status.contains(Status::CARRY) { self.jmp(addr.unwrap()) }, /* BCS */
                0x90 => if !self.status.contains(Status::CARRY) { self.jmp(addr.unwrap()) }, /* BCC */
                0xf0 => if self.status.contains(Status::ZERO) { self.jmp(addr.unwrap()) }, /* BEQ */
                0xd0 => if !self.status.contains(Status::ZERO) { self.jmp(addr.unwrap()) }, /* BNE */
                0x30 => if self.status.contains(Status::NEGATIVE) { self.jmp(addr.unwrap()) }, /* BMI */
                0x10 => if !self.status.contains(Status::NEGATIVE) {self.jmp(addr.unwrap()) }, /* BPL */
                0x70 => if self.status.contains(Status::OVERFLOW) { self.jmp(addr.unwrap()) }, /* BVS */
                0x50 => if !self.status.contains(Status::OVERFLOW) { self.jmp(addr.unwrap()) }, /* BVC */
                _ => panic!("Unimplemented opcode: {:02x}", code),
            }

            callback(self);
        }
    }

    fn addition(&mut self, val: u8) -> u8 {
        let mut sum = self.register_a as u16 + val as u16;
        
        if self.status.contains(Status::CARRY) {
            sum += 1;
        }

        if sum > 0xff {
            self.status.insert(Status::CARRY);
        } else {
            self.status.remove(Status::CARRY);
        }

        let res = sum as u8; 
        // http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        if (res ^ self.register_a) & (res ^ val) & 0x80 != 0 {
            self.status.insert(Status::OVERFLOW);
        } else { 
            self.status.remove(Status::OVERFLOW);
        }

        res
    }

    fn adc(&mut self, addr: u16) {
        let res = self.addition(self.mem_read(addr));
        self.register_a = res;
        self.update_zero_and_negative_flag(res);
    }

    fn sbc(&mut self, addr: u16) {
        let res = self.addition(self.mem_read(addr).wrapping_neg().wrapping_sub(1) as u8);
        self.register_a = res;
        self.update_zero_and_negative_flag(res);
    }

    fn jsr(&mut self, addr: u16) {
        self.stack_push_u16(self.program_counter.wrapping_sub(1));
        self.program_counter = addr;
    }

    fn rts(&mut self) {
        self.program_counter = self.stack_pop_u16().wrapping_add(1);
    }

    fn rti(&mut self) {
        self.status = Status::from_bits_truncate(self.stack_pop_u8());
        self.program_counter = self.stack_pop_u16();
    }

    fn bit(&mut self, addr: u16) {
        let value = self.mem_read(addr);
        self.status.set(Status::ZERO, (self.register_a & value) == 0);
        self.status.set(Status::NEGATIVE, value & 0x80 != 0);
        self.status.set(Status::OVERFLOW, value & 0x40 != 0);
    }

    fn rol_acc(&mut self) {
        let val = self.rol(self.register_a);
        self.register_a = val;
    }

    fn rol_mem(&mut self, addr: u16) {
        let val = self.rol(self.mem_read(addr));
        self.mem_write(addr, val);
    }

    fn rol(&mut self, val: u8) -> u8 {
        let old_carry = self.status.contains(Status::CARRY) as u8;
        self.status.set(Status::CARRY, val & 0x80 != 0);

        let new_val = (val << 1) | old_carry;
        self.update_zero_and_negative_flag(new_val);
        new_val
    }

    fn ror_acc(&mut self) {
        let val = self.ror(self.register_a);
        self.register_a = val;
    }

    fn ror_mem(&mut self, addr: u16) {
        let val = self.ror(self.mem_read(addr));
        self.mem_write(addr, val);
    }

    fn ror(&mut self, val: u8) -> u8 {
        let old_carry = self.status.contains(Status::CARRY) as u8;
        self.status.set(Status::CARRY, val & 0x01 != 0);

        let new_val = (val >> 1) | (old_carry << 7);
        self.update_zero_and_negative_flag(new_val);
        new_val
    }

    fn asl_acc(&mut self) {
        let val = self.asl(self.register_a);
        self.register_a = val;
    }

    fn asl_mem(&mut self, addr: u16) {
        let val = self.asl(self.mem_read(addr));
        self.mem_write(addr, val);
    }

    fn asl(&mut self, val: u8) -> u8 {
        self.status.set(Status::CARRY, val & 0x80 != 0);
        let new_val = val << 1;
        self.update_zero_and_negative_flag(new_val);
        new_val
    }

    fn lsr_acc(&mut self) {
        let val = self.lsr(self.register_a);
        self.register_a = val;
    }

    fn lsr_mem(&mut self, addr: u16) {
        let val = self.lsr(self.mem_read(addr));
        self.mem_write(addr, val);
    }

    fn lsr(&mut self, val: u8) -> u8 {
        self.status.set(Status::CARRY, val & 0x01 != 0);
        let new_val = val >> 1;
        self.update_zero_and_negative_flag(new_val);
        new_val
    }

    /*
    An original 6502 has does not correctly fetch the target address if the 
    indirect vector falls on a page boundary (e.g. $xxFF where xx is any value from $00 to $FF). 
    In this case fetches the LSB from $xxFF as expected but takes the MSB from $xx00.
    https://www.nesdev.org/obelisk-6502-guide/reference.html#JMP
     */
    fn jmp_ind(&mut self) {
        let pos = self.mem_read_u16(self.program_counter);
        let address = if pos & 0x00ff == 0x00ff {
            u16::from_le_bytes([
                self.mem_read(pos),
                self.mem_read(pos & 0xff00)
            ])
        } else {
            self.mem_read_u16(pos)
        };

        self.jmp(address);
    }

    fn jmp(&mut self, addr: u16) {
        self.program_counter = addr;
    }

    fn pha(&mut self) {
        self.stack_push_u8(self.register_a);
    }

    fn php(&mut self) {
        self.stack_push_u8(self.status.bits());
    }

    fn pla(&mut self) {
        self.register_a = self.stack_pop_u8();
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn plp(&mut self) {
        self.status = Status::from_bits_truncate(self.stack_pop_u8());
    }

    fn clc(&mut self) {
        self.status.remove(Status::CARRY)
    }

    fn cld(&mut self) {
        self.status.remove(Status::DECIMAL)
    }

    fn cli(&mut self) {
        self.status.remove(Status::INTERDIS)
    }

    fn clv(&mut self) {
        self.status.remove(Status::OVERFLOW)
    }

    fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flag(self.register_y);
    }

    fn tsx(&mut self) {
        self.register_x = self.stack_pointer;
        self.update_zero_and_negative_flag(self.register_x);
    }

    fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn txs(&mut self) {
        self.stack_pointer = self.register_x;
        self.update_zero_and_negative_flag(self.stack_pointer);
    }

    fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn sed(&mut self) {
        self.status.insert(Status::DECIMAL);
    }

    fn sei(&mut self) {
        self.status.insert(Status::INTERDIS);
    }

    fn sec(&mut self) {
        self.status.insert(Status::CARRY);
    }

    fn and(&mut self, addr: u16) {
        let value = self.mem_read(addr);
        self.register_a &= value;
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn ora(&mut self, addr: u16) {
        let value = self.mem_read(addr);
        self.register_a |= value;
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn eor(&mut self, addr: u16) {
        let value = self.mem_read(addr);
        self.register_a ^= value;
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn ldx(&mut self, addr: u16) {
        let value = self.mem_read(addr);
        self.register_x = value;
        self.update_zero_and_negative_flag(self.register_x);
    }

    fn ldy(&mut self, addr: u16) {
        let value = self.mem_read(addr);
        self.register_y = value;
        self.update_zero_and_negative_flag(self.register_y);
    }

    fn stx(&mut self, addr: u16) {
        self.mem_write(addr, self.register_x)
    }

    fn sta(&mut self, addr: u16) {
        self.mem_write(addr, self.register_a)
    }

    fn sty(&mut self, addr: u16) {
        self.mem_write(addr, self.register_y)
    }

    fn inc(&mut self, addr: u16) {
        let value = self.mem_read(addr).wrapping_add(1);
        self.mem_write(addr, value);
        self.update_zero_and_negative_flag(value);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flag(self.register_x);
    }

    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flag(self.register_y);
    }

    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flag(self.register_x);
    }

    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flag(self.register_y);
    }

    fn dec(&mut self, addr: u16) {
        let value = self.mem_read(addr).wrapping_sub(1);
        self.mem_write(addr, value);
        self.update_zero_and_negative_flag(value);
    }

    fn cmp(&mut self, addr: u16) {
        let value = self.mem_read(addr);
        let result = self.register_a.wrapping_sub(value);

        self.update_zero_and_negative_flag(result);
        self.update_carry_flag(self.register_a, value);
    }

    fn cpx(&mut self, addr: u16) {
        let value = self.mem_read(addr);
        let result = self.register_x.wrapping_sub(value);

        self.update_zero_and_negative_flag(result);
        self.update_carry_flag(self.register_x, value);
    }

    fn cpy(&mut self, addr: u16) {
        let value = self.mem_read(addr);
        let result = self.register_y.wrapping_sub(value);

        self.update_zero_and_negative_flag(result);
        self.update_carry_flag(self.register_y, value);
    }

    fn lda(&mut self, addr: u16) {
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flag(self.register_x);
    }

    fn next(&mut self) -> u8 {
        let value = self.mem_read(self.program_counter);
        self.program_counter += 1;
        value
    }

    fn update_zero_flag(&mut self, value: u8) {
        if value == 0 {
            self.status.insert(Status::ZERO)
        } else {
            self.status.remove(Status::ZERO)
        }
    }

    fn update_negative_flag(&mut self, value: u8) {
        if value & 0b1000_0000 != 0 { 
            self.status.insert(Status::NEGATIVE)
        } else { // 6502 Integers are neither signed or unsigned. Neg depends on the most significant bit.
            self.status.remove(Status::NEGATIVE)
        }
    }

    fn update_carry_flag(&mut self, v1: u8, v2: u8) {
        if v1 >= v2 {
            self.status.insert(Status::CARRY)
        } else {
            self.status.remove(Status::CARRY)
        }
    }

    fn update_zero_and_negative_flag(&mut self, value: u8) {
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }
}