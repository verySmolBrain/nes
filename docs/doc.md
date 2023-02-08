# Overview of Emulator

## CPU 

* 16-bit for memory addressing
* 6502 Processor without Decimal

### CPU Memory Map
```  _______________ $10000  _______________
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
```

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
* 1-bit Control Bus -> Read or write access
* 16-bit Address Bus -> Address of required location

The CPU memory map is divided into 3 parts. The Zero Page [0000 .. 0100],
the Stack [0100 .. 0200] and the rest of the RAM [0200 .. 0800]. This is
mirrored in the address space [0x0000 .. 0x2000].

The reason for the mirroring is that the CPU ram chip only has address space [0x0000 .. 0x0800] (11 bits) but the chip used to decode the address space only uses the first 13 bits.
With the address bus possessing 16-bits, there are 13 bits left with an address space of
only 11 bits. Decoding the address space requires extra logical resources so the NES
Motherboard has only 11 addressing tracks and simply ignores the 2 MSB. This also
applies to the PPU registers.

## ROM Catride Structure

* The physical cartridges had 2 banks of ROM memory.
PRG ROM for code & CHR ROM for graphics. PRG ROM gets 
connected to CPU and CHR ROM gets connected to PPU
so neither can access each other. 

* We deal with ROM dumps which are slightly different. The most popular is iNES
which follows the following format:

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

   Pattern Table -> CHR ROM
   Name Tables -> VRAM
```

The PPU has its own set of registers. Namely:

* Controller (0x2000) -> General logic flow
```
7  bit  0
---- ----
VPHB SINN
|||| ||||
|||| ||++- Base nametable address
|||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
|||| |+--- VRAM address increment per CPU read/write of PPUDATA
|||| |     (0: add 1, going across; 1: add 32, going down)
|||| +---- Sprite pattern table address for 8x8 sprites
||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
|||+------ Background pattern table address (0: $0000; 1: $1000)
||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels – see PPU OAM#Byte 1)
|+-------- PPU master/slave select
|          (0: read backdrop from EXT pins; 1: output color on EXT pins)
+--------- Generate an NMI at the start of the
           vertical blanking interval (0: off; 1: on)
```

* Mask (0x2001) -> Rendering method for sprites and background

```
7  bit  0
---- ----
BGRs bMmG
|||| ||||
|||| |||+- Greyscale (0: normal color, 1: produce a greyscale display)
|||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
|||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
|||| +---- 1: Show background
|||+------ 1: Show sprites
||+------- Emphasize red (green on PAL/Dendy)
|+-------- Emphasize green (red on PAL/Dendy)
+--------- Emphasize blue
```

* Status (0x2002) -> PPU Status

```
7  bit  0
---- ----
VSO. ....
|||| ||||
|||+-++++- PPU open bus. Returns stale PPU bus contents.
||+------- Sprite overflow. The intent was for this flag to be set
||         whenever more than eight sprites appear on a scanline, but a
||         hardware bug causes the actual behavior to be more complicated
||         and generate false positives as well as false negatives; see
||         PPU sprite evaluation. This flag is set during sprite
||         evaluation and cleared at dot 1 (the second dot) of the
||         pre-render line.
|+-------- Sprite 0 Hit.  Set when a nonzero pixel of sprite 0 overlaps
|          a nonzero background pixel; cleared at dot 1 of the pre-render
|          line.  Used for raster timing.
+--------- Vertical blank has started (0: not in vblank; 1: in vblank).
           Set at dot 1 of line 241 (the line *after* the post-render
           line); cleared after reading $2002 and at dot 1 of the
           pre-render line.
```

* OAM Address (0x2003) -> (Object Attribute Memory) -> Space for Sprites
* OAM Data (0x2004) -> Data at OAM Address
* Scroll (0x2005) -> Viewport
* Address (0x2006) -> Address of PPU Memory
* Data (0x2007) -> Data at PPU Memory
* OAM DMA (0x2008) -> (Fast Copying of 256 bytes from CPU RAM to OAM) -> Direct Memory Access

This is stored in the CPU in the address space `[0x2000 .. 0x2008]` Which is mirrored
every 8 bytes in the address space `[0x2008 .. 0x4000]` (This is because an NES uses a cheap
decoder chip which only has 11 address lines and it is easier and faster to simply decode
the address incompletely).

The PPU itself has its own address space as seen above which is accessed by the CPU through
the PPU registers. The address space is `[0x0000 .. 0x3FFF]` which is mirrored throughout
`[0x0000 .. 0xFFFF]`. The PPU can also notify the CPU through NMI interrupts.

Extras:

* Latch -> Commonly used in chips for temporary storage. Both scroll and address take advantage of latches to temporarily store some type of data.
* Mirroring -> Since the PPU has 2kb of VRAM and the NES uses 1kb of VRAM to represent a single screen, an NES
PPU can represent the state of 2 screens. However the PPU memory map has range `[0x2000 .. 0x3F00]` which is
enough to fit 4 screens. This means we still have to map another 2 screens. How this is mapped is 
decided by the mirroring in the NES ROM Header. (This has to do with scrolling since you need another
screen as a sort of 'placeholder' for when you scroll past the edge of the screen.)
* Buffer -> As CHR ROM and VRAM are considered separate from PPU the PPU actually stores the result of the data
it reads into a buffer. This means the PPU is perpetually one read behind the actual value.

PPU Rendering

* The PPU renders 262 scanlines per frame. Each scanline lasts for 341 PPU clock cycles. There
are 240 scanlines used for rendering and the rest (241 - 262) is used for vertical blanking.
* Vertical Blanking is the period where the NES PPU is not drawing to the screen. This is the 
time where the PPU can send an NMI Interrupt to the CPU to allow the CPU to do various other tasks.

## Interrupts

## Important Notes
- Address is stored in 2 bytes
- [Little endian](https://stackoverflow.com/questions/4752715/why-are-both-little-and-big-endian-in-use) is used for addresses


[Reset](https://en.wikipedia.org/wiki/Reset_vector) vector is located in `ROM` since:
1. You don't want to conflict with `RAM` space
2. `NES ROM` cartridges are probably not going to be big enough to cover the
address space anyways so the top of `ROM` is good place for instructions
