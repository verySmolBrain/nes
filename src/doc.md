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
- Stack Pointer - Memory space [0x0100 .. 0x1FF]. Holds the address of the top (Grows from top to bottom) -> Doesn't overflow only wraps around
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
- 1 - Unused flag that is always set to 1

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

## Important Notes
- Address is stored in 2 bytes
- [Little endian](https://stackoverflow.com/questions/4752715/why-are-both-little-and-big-endian-in-use) is used for addresses


[Reset](https://en.wikipedia.org/wiki/Reset_vector) vector is located in `ROM` since:
1. You don't want to conflict with `RAM` space
2. `NES ROM` cartridges are probably not going to be big enough to cover the
address space anyways so the top of `ROM` is good place for instructions
