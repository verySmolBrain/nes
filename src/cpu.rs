use crate::opcodes::OPCODES;

const ADDRESS_SPACE: usize = 0xFFFF;
const ROM_START: usize = 0x8000;
const RESET_VECTOR: usize = 0xFFFC;

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
    Indirect_X, // Change name since this is meant to be Indexed Indirect
    Indirect_Y,
    NoneAddressing,
}

pub struct CPU {
    pub register_a: u8, 
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; ADDRESS_SPACE]
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0, 
            register_x: 0,
            register_y: 0,
            status: 0, 
            program_counter: 0,
            memory: [0; ADDRESS_SPACE]
        }
    }

    fn mem_read_u16(&self, addr: u16) -> u16 {
        u16::from_le_bytes([ // LE
            self.mem_read(addr),
            self.mem_read(addr + 1)
        ])
    }

    fn mem_write_u16(&mut self, addr: u16, value: u16) {
        value.to_le_bytes().iter().enumerate().for_each(|(i, v)| {
            self.mem_write(addr + i as u16, *v) // LE
        })
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x);
                addr as u16
            },
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y);
                addr as u16
            },
            AddressingMode::Absolute_X => {
                let pos = self.mem_read_u16(self.program_counter);
                let addr = pos.wrapping_add(self.register_x as u16);
                addr
            },
            AddressingMode::Absolute_Y => {
                let pos = self.mem_read_u16(self.program_counter);
                let addr = pos.wrapping_add(self.register_y as u16);
                addr
            },
            AddressingMode::Indirect_X => {
                let pos = self.mem_read(self.program_counter);
                let ptr = pos.wrapping_add(self.register_x);

                u16::from_le_bytes([ // Indexed Indirect adding before lookup
                    self.mem_read(ptr as u16),
                    self.mem_read(ptr.wrapping_add(1) as u16)
                ])
            },
            AddressingMode::Indirect_Y => {
                let pos = self.mem_read(self.program_counter);
                let ptr = u16::from_le_bytes([
                    self.mem_read(pos as u16),
                    self.mem_read(pos.wrapping_add(1) as u16)
                ]); // Indirect Index adding after lookup

                let addr = ptr.wrapping_add(self.register_y as u16);
                addr
            },
            AddressingMode::NoneAddressing => {
                panic!("Invalid mode: {:?}", mode);
            }
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(RESET_VECTOR as u16);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[ROM_START .. (ROM_START + program.len())]
            .copy_from_slice(&program[..]);
        self.mem_write_u16(RESET_VECTOR as u16, ROM_START as u16)
    }

    pub fn run(&mut self) {
        loop {
            let code = self.next();
            let opcode = OPCODES.get(&code).expect("Invalid opcode");

            match opcode.opcode {
                0xA9 => { /* LDA */
                    self.lda(
                        self.get_operand_address(&opcode.mode)
                    );
                },
                0xE8 => self.inx(), /* INX */
                0xAA => self.tax(), /* TAX */
                0x00 => return, /* BRK */
                _ => todo!()
            }

            self.program_counter += opcode.bytes - 1;
        }
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flag(self.register_x);
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
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }
    }

    fn update_negative_flag(&mut self, value: u8) {
        if value & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }

    fn update_zero_and_negative_flag(&mut self, value: u8) {
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }
}