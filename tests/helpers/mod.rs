use nes::cpu::Cpu;
use nes::rom::{ Rom, PRG_ROM_PAGE_SIZE, CHR_ROM_PAGE_SIZE, Mirroring };
use nes::memory::Mem;
use nes::cpu::Status;
use nes::opcodes::OPCODES;
use nes::cpu::AddressingMode;

const HEADER_LEN: usize = 16;

pub fn check_zero_and_negative(cpu: Cpu, value: u8) {
    if value == 0 {
        assert!(cpu.status.contains(Status::ZERO));
    } else {
        assert!(!cpu.status.contains(Status::ZERO));
    }
    if value & 0b1000_0000 != 0 {
        assert!(cpu.status.contains(Status::NEGATIVE));
    } else {
        assert!(!cpu.status.contains(Status::NEGATIVE));
    }
}

pub struct TestRom {
    pub nes_header: [u8; 4],
    pub num_prg_rom: u8,
    pub num_chr_rom: u8,
    pub num_prg_ram: u8,
    pub flags_6: u8,
    pub flags_7: u8,
    pub trainer: Option<Vec<u8>>,
}

impl TestRom {
    pub fn get_header(&self) -> [u8; 16] {
        let mut header = [0; 16];
        header[0..4].copy_from_slice(&self.nes_header);
        header[4] = self.num_prg_rom;
        header[5] = self.num_chr_rom;
        header[6] = self.flags_6;
        header[7] = self.flags_7;
        header[8] = self.num_prg_ram;
        header
    }

    pub fn get_size(&self) -> usize {
        let mut size = HEADER_LEN;
        if self.trainer.is_some() {
            size += 512;
        }
        size += self.num_prg_rom as usize * PRG_ROM_PAGE_SIZE;
        size += self.num_chr_rom as usize * CHR_ROM_PAGE_SIZE;
        size
    }

    pub fn test_rom_raw(&self) -> Vec<u8> {
        let mut rom = Vec::with_capacity(self.get_size());
        rom.extend_from_slice(&self.get_header());
        if let Some(trainer) = &self.trainer {
            rom.extend_from_slice(trainer);
        }
        rom.extend(vec![1; self.num_prg_rom as usize * PRG_ROM_PAGE_SIZE]);
        rom.extend(vec![2; self.num_chr_rom as usize * CHR_ROM_PAGE_SIZE]);
        rom
    }

    pub fn default_rom() -> Rom {
        Rom {
            prg_rom: vec![1; 2 * PRG_ROM_PAGE_SIZE],
            chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
            mapper: 0,
            screen_mirroring: Mirroring::VERTICAL,
        }
    }
}

pub fn trace(cpu: &mut Cpu) -> String {
    let pc = cpu.program_counter;

    let code = cpu.mem_read(pc);
    let opcode = OPCODES.get(&code).expect("Invalid opcode");

    let mut lsb: Option<u8> = None;
    let mut msb: Option<u8> = None;
    let mut assembly_translation: Option<String> = None;

    let accumulator = cpu.register_a;
    let x = cpu.register_x;
    let y = cpu.register_y;
    let status = cpu.status.bits();
    let sp = cpu.stack_pointer;

    match opcode.mode {
        AddressingMode::Implied => {
            assembly_translation = Some(format!("{}", opcode.name))
        },
        AddressingMode::Immediate => {
            lsb = Some(cpu.mem_read(pc.wrapping_add(1)));
            assembly_translation = Some(format!("{} #${:02X}", opcode.name, lsb.unwrap()))
        },
        AddressingMode::Relative => {
            lsb = Some(cpu.mem_read(pc.wrapping_add(1)));
            let address_after_offset = pc.wrapping_add(1).wrapping_add(1).wrapping_add(lsb.unwrap() as u16);
            assembly_translation = Some(format!("{} ${:02X}", opcode.name, address_after_offset))
        },
        AddressingMode::Accumulator => {
            assembly_translation = Some(format!("{} A", opcode.name))
        },
        AddressingMode::ZeroPage => {
            lsb = Some(cpu.mem_read(pc.wrapping_add(1)));
            assembly_translation = Some(format!("{} ${:02X}", opcode.name, lsb.unwrap()))
        },
        AddressingMode::ZeroPage_X => {
            lsb = Some(cpu.mem_read(pc.wrapping_add(1)));
            let offset = lsb.unwrap().wrapping_add(x);
            let val_at_offset = cpu.mem_read(offset as u16);
            assembly_translation = Some(format!("{} ${:02X},X @ {:02X} = {:02X}", opcode.name, lsb.unwrap(), offset, val_at_offset))
        },
        AddressingMode::ZeroPage_Y => {
            lsb = Some(cpu.mem_read(pc.wrapping_add(1)));
            let offset = lsb.unwrap().wrapping_add(y);
            let val_at_offset = cpu.mem_read(offset as u16);
            assembly_translation = Some(format!("{} ${:02X},Y @ {:02X} = {:02X}", opcode.name, lsb.unwrap(), offset, val_at_offset))
        },
        AddressingMode::Absolute => {
            lsb = Some(cpu.mem_read(pc.wrapping_add(1)));
            msb = Some(cpu.mem_read(pc.wrapping_add(2)));
            assembly_translation = Some(format!("{} ${:02X}{:02X}", opcode.name, msb.unwrap(), lsb.unwrap()))
        },
        AddressingMode::Absolute_X => {
            lsb = Some(cpu.mem_read(pc.wrapping_add(1)));
            msb = Some(cpu.mem_read(pc.wrapping_add(2)));

            let address = u16::from_le_bytes([lsb.unwrap(), msb.unwrap()]);
            let address_after_offset = address.wrapping_add(x as u16);
            let val_at_offset = cpu.mem_read(address_after_offset);

            assembly_translation = Some(format!("{} ${:02X}{:02X},X @ {:04X} = {:02X}", opcode.name, msb.unwrap(), lsb.unwrap(), address_after_offset, val_at_offset))
        },
        AddressingMode::Absolute_Y => {
            lsb = Some(cpu.mem_read(pc.wrapping_add(1)));
            msb = Some(cpu.mem_read(pc.wrapping_add(2)));

            let address = u16::from_le_bytes([lsb.unwrap(), msb.unwrap()]);
            let address_after_offset = address.wrapping_add(y as u16);
            let val_at_offset = cpu.mem_read(address_after_offset);

            assembly_translation = Some(format!("{} ${:02X}{:02X},Y @ {:04X} = {:02X}", opcode.name, msb.unwrap(), lsb.unwrap(), address_after_offset, val_at_offset))
        },
        AddressingMode::Indirect_Y => {
            lsb = Some(cpu.mem_read(pc.wrapping_add(1)));

            let address_at_offset = u16::from_le_bytes([
                cpu.mem_read(lsb.unwrap() as u16),
                cpu.mem_read(lsb.unwrap().wrapping_add(1) as u16)
            ]);
            let address_after_offset = address_at_offset.wrapping_add(y as u16);
            let value_at_address = cpu.mem_read(address_after_offset);

            assembly_translation = Some(format!("{} (${:02X}),Y = {:04X} @ {:04X} = {:02X}", opcode.name, lsb.unwrap(), address_at_offset, address_after_offset, value_at_address))
        },
        AddressingMode::Indirect_X => {
            lsb = Some(cpu.mem_read(pc.wrapping_add(1)));

            let address_after_offset = lsb.unwrap().wrapping_add(x);
            let address_at_offset = u16::from_le_bytes([
                cpu.mem_read(address_after_offset as u16),
                cpu.mem_read(address_after_offset.wrapping_add(1) as u16)
            ]);
            let value_at_address = cpu.mem_read(address_at_offset);

            assembly_translation = Some(format!("{} (${:02X},X) @ {:02X} = {:04X} = {:02X}", opcode.name, lsb.unwrap(), address_after_offset, address_at_offset, value_at_address))
        },
        AddressingMode::JMPIndirect => { // Solely for JMP Indirect
            lsb = Some(cpu.mem_read(pc.wrapping_add(1)));
            msb = Some(cpu.mem_read(pc.wrapping_add(2)));

            let address = u16::from_le_bytes([lsb.unwrap(), msb.unwrap()]);

            let address_after_boundary = if lsb == Some(0xFF) {
                u16::from_le_bytes([
                    cpu.mem_read(address),
                    cpu.mem_read(address & 0xFF00)
                ])
            } else {
                cpu.mem_read_u16(address)
            };

            let value_at_address = cpu.mem_read(address_after_boundary);

            assembly_translation = Some(format!("{} (${:02X}{:02X}) = {:04X}", opcode.name, msb.unwrap(), lsb.unwrap(), value_at_address))
        },
        AddressingMode::NoneAddressing => {
            
        }
    }

    let hex = vec![Some(code), lsb, msb];
    let hex_str = hex.iter()
        .map(|x| match x {
            Some(x) => format!("{:02X}", x),
            None => "".to_string()
        })
        .collect::<Vec<String>>()
        .join(" ");

    format!("{:04X}  {:<8}  {:<30}  A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
        pc, 
        hex_str,
        assembly_translation.unwrap_or("".to_string()),
        accumulator,
        x,
        y,
        status,
        sp
    )
}