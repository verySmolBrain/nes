use nes::emulator::cpu::Cpu;
use nes::emulator::bus::Bus;
use nes::emulator::memory::Mem;
use nes::emulator::rom::{ Rom, PRG_ROM_PAGE_SIZE, CHR_ROM_PAGE_SIZE, Mirroring };
use expect_test::Expect;
use nes::helpers::trace::trace;

const HEADER_LEN: usize = 16;

pub fn check(cpu: &mut Cpu, expected: Expect) {
    let mut res: Vec<String> = vec![];
    cpu.run_with_callback(|cpu| {
        res.push(trace(cpu));
    });

    expected.assert_eq(&res.join("\n"));
}

pub fn load_into_memory(bus: &mut Bus, data: Vec<u8>, start: u16) {
    for (i, byte) in data.iter().enumerate() {
        bus.mem_write(start + i as u16, *byte);
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

