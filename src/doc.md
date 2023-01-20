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
NES Header 16 Bytes
Trainer (Optional) 512 Bytes
PRG ROM 16384 * x Bytes (x depends on header)
CHR ROM 8192 * y Bytes (y depends on header)
```

```
NES Header
0-3 - Constant $4E $45 $53 $1A (ASCII "NES" followed by MS-DOS end-of-file)
4 - Size of PRG ROM in 16 KB units
5 - Size of CHR ROM in 8 KB units (Value 0 means the board uses CHR RAM)
6 - Flags 6 (mapper, mirroring, battery, trainer)
7 - Flags 7 (mapper, VS/Playchoice, NES 2.0)
8 - Flags 8 (PRG-RAM size)
9 - Flags 9 (TV system, PRG-RAM presence, bus conflicts)
10 - Flags 10 (TV system, PRG-RAM presence, bus conflicts)
11-15 - Unused padding (should be filled with zero, but some rippers put their name across bytes 7-15)

Flags 8 - 15 are rarely used.
```

##

## Important Notes
- Address is stored in 2 bytes
- [Little endian](https://stackoverflow.com/questions/4752715/why-are-both-little-and-big-endian-in-use) is used for addresses


[Reset](https://en.wikipedia.org/wiki/Reset_vector) vector is located in `ROM` since:
1. You don't want to conflict with `RAM` space
2. `NES ROM` cartridges are probably not going to be big enough to cover the
address space anyways so the top of `ROM` is good place for instructions
