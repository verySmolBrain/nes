# Overview of Emulator

## CPU 

- 16-bit for memory addressing
- 6502 Processor without Decimal

### CPU Memory Map
- RAM - [0x0000 … 0x2000]
- NES hardware modules: PPU, APU, GamePads - [0x2000 … 0x4020]
- Mappers - [0x4020 .. 0x6000]
- Cartridge RAM - [0x6000 .. 0x8000]
- Program ROM on Cartridge - [0x8000 … 0x10000]

### CPU Registers

Program Counter is 16 bits. Others are 8 bits.
- Program Counter (PC) - holds address for next machine language instruction
- Stack Pointer - Memory space [0x0100 .. 0x01FF]. Holds the address of the top (Grows from top to bottom) -> Doesn't overflow only wraps around
- Accumulator (A) - stores results of arithmetic, logic and memory access operations 
- Index Register X (X) - used as an offset in specific memory addressing modes. Can be used for auxiliary storage needs (Temp values etc.)
- Index Register Y (Y) - same as X 
- Processor Status (P) - represents 7 status flags that can be set or unset
Processor Status Flags
- N - Negative Flag - Set after any operation
- V - Overflow Flag
- B - Break Flag - Distinguish hardware interrupts from software interrupts
- D - Decimal Flag - Select Decimal mode 
- I - Interrupt Disable Flag - Disable CPU interrupts
- Z - Zero Flag - Set if last operation result was 0 
- C - Carry Flag - Carryover for bigger than 8-bit numbers

```
7  bit  0
---- ----
NVss DIZC
|||| ||||
|||| |||+- Carry
|||| ||+-- Zero
|||| |+--- Interrupt Disable
|||| +---- Decimal
||++------ No CPU effect, see: the B flag
|+-------- Overflow
+--------- Negative
```

### Addressing Modes

- Absolute -> Whole address space is used (2 bytes)
- Absolute X -> Absolute but register x is added
- Absolute Y -> Absolute but register y is added
- Zero Page -> Absolute but smol (Only 1 byte for first page)
- Zero Page X -> Zero page but we also add value in register x and if the 
result is more than 1 byte, we wrap around (Offset to jump to memory location
can be quite efficient eg. Arrays)
- Zero Page Y -> Zero Page X but we use register y instead
- Immediate -> Data is directly given as byte
- Indirect -> Data is accessed using a pointer. (Makes implementation of 
pointers or routines easier but is less performant)
- Indexed Indirect -> Zero Page Address + X Register -> Look up 2-byte address
- Indirect Indexed -> Zero Page Address -> Look by Address + Y Register

[Addressing Modes](https://wiki.cdot.senecacollege.ca/wiki/6502_Addressing_Modes)


### Bus

* 8-bit Data Bus -> Byte being read or written
* 1-bit Control But -> Read or write access
* 16-bit Address Bus -> Address of required location

Mirroring

* [0x000 .. 0x0800 ]
* [0x800 .. 0x1000 ]
* [0x1000 .. 0x1800 ]
* [0x1800 .. 0x2000 ]

* NES Motherboard has only 11 addressing tracks but the 
addressing space reserved for RAM [0x0000 .. 0x2000 ] has 13 bits.
So we effectively need to Zero out the 2 MSB if we receive a 
request in the RAM address space.

* This also applies to address space [ 0x2008 .. 0x4000 ] which mirrors 
memory mappings for PPU registers [ 0x2000 .. 0x2008 ].

## ROM Catride Structure

* The physical cartridges had 2 banks of ROM memory.
PRG ROM for code & CHR ROM for graphics. PRG ROM gets 
connected to CPU and CHR ROM gets connected to PPU
so neither can access each other. 

* We deal with ROM dumps which are slightly different. The most popular is iNES.

```
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
```

```
    Flags 6 

    76543210
    ||||||||
    |||||||+- Mirroring: 0: horizontal (vertical arrangement) (CIRAM A10 = PPU A11)
    |||||||              1: vertical (horizontal arrangement) (CIRAM A10 = PPU A10)
    ||||||+-- 1: Cartridge contains battery-backed PRG RAM ($6000-7FFF) or other persistent memory
    |||||+--- 1: 512-byte trainer at $7000-$71FF (stored before PRG data)
    ||||+---- 1: Ignore mirroring control or above mirroring bit; instead provide four-screen VRAM
    ++++----- Lower nybble of mapper number
```

```
    Flags 7
    
    76543210
    ||||||||
    |||||||+- VS Unisystem
    ||||||+-- PlayChoice-10 (8 KB of Hint Screen data stored after CHR data)
    ||||++--- If equal to 2, flags 8-15 are in NES 2.0 format
    ++++----- Upper nybble of mapper number
```

## PPU

```
    PPU Memory Map
    _______________ $3FFF  _______________
   | Palette RAM   |       |               |
   |_______________| $3F00 |_______________|
   | Mirrors       |       | Palette RAM   |
   | $3F00-$3F1F   |       |               |
   |_______________| $3F20 |_______________|
   | Nametable RAM |       | Nametable RAM |
   |_______________| $3000 |_______________|
   | Mirrors       |       | Nametable RAM |
   | $2000-$2FFF   |       |_______________|
   |_______________| $2000 |_______________|
   | Pattern Table |       | Pattern Table |
   |_______________| $1000 |_______________|
   | Pattern Table |       | Pattern Table |
   |_ _ _ _ _ _ _ _| $0000 |_______________|
```

## Important Notes
- Address is stored in 2 bytes
- [Little endian](https://stackoverflow.com/questions/4752715/why-are-both-little-and-big-endian-in-use) is used for addresses


[Reset](https://en.wikipedia.org/wiki/Reset_vector) vector is located in `ROM` since:
1. You don't want to conflict with `RAM` space
2. `NES ROM` cartridges are probably not going to be big enough to cover the
address space anyways so the top of `ROM` is good place for instructions
