/*
    Map for NES ROM header
    + - - - - - - - - - - - Constant ($4E $45 $53 $1A) used to identify NES file
    | | | | + - - - - - - - Number of 16 KB PRG-ROM banks
    | | | | | + - - - - - - Number of 8 KB CHR-ROM banks
    | | | | | | + - - - - - Flags 6
    | | | | | | | + - - - - Flags 7
    | | | | | | | | + - - - Size of 8 KB PRG-RAM banks
    | | | | | | | | | + - - Flags 9 (unused)
    | | | | | | | | | |
    N N N N P C B B R 0 0 0 0 0 0 0
 */

/*
    Flags 6 

    76543210
    ||||||||
    |||||||+- Mirroring: 0: horizontal (vertical arrangement) (CIRAM A10 = PPU A11)
    |||||||              1: vertical (horizontal arrangement) (CIRAM A10 = PPU A10)
    ||||||+-- 1: Cartridge contains battery-backed PRG RAM ($6000-7FFF) or other persistent memory
    |||||+--- 1: 512-byte trainer at $7000-$71FF (stored before PRG data)
    ||||+---- 1: Ignore mirroring control or above mirroring bit; instead provide four-screen VRAM
    ++++----- Lower nybble of mapper number
  */

/*
    Flags 7
    
    76543210
    ||||||||
    |||||||+- VS Unisystem
    ||||||+-- PlayChoice-10 (8 KB of Hint Screen data stored after CHR data)
    ||||++--- If equal to 2, flags 8-15 are in NES 2.0 format
    ++++----- Upper nybble of mapper number
 */