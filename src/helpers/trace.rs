use crate::opcodes::OPCODES;
use crate::cpu::AddressingMode;
use crate::cpu::Cpu;
use crate::memory::Mem;

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
            let value_at_address = cpu.mem_read(lsb.unwrap() as u16);
            assembly_translation = Some(format!("{} ${:02X} = {:02X}", opcode.name, lsb.unwrap(), value_at_address))
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
            let address = u16::from_le_bytes([lsb.unwrap(), msb.unwrap()]);
            let val_at_address = cpu.mem_read(address);
            assembly_translation = Some(format!("{} ${:02X}{:02X} = {:02X}", opcode.name, msb.unwrap(), lsb.unwrap(), val_at_address))
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

            assembly_translation = Some(format!("{} (${:02X}{:02X}) = {:04X}", opcode.name, msb.unwrap(), lsb.unwrap(), address_after_boundary))
        },
        AddressingMode::Jump => {
            lsb = Some(cpu.mem_read(pc.wrapping_add(1)));
            msb = Some(cpu.mem_read(pc.wrapping_add(2)));
            assembly_translation = Some(format!("{} ${:02X}{:02X}", opcode.name, msb.unwrap(), lsb.unwrap()))
        },
        AddressingMode::NoneAddressing => {
            
        },
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