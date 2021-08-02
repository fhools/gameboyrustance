
# Resources 
[GBATEK](http://problemkaputt.de/gbatek.htm)

[Emulation Accuray Speed and Optimization](https://mgba.io/2017/04/30/emulation-accuracy/)

[Gameboy Advanced Architecture - An Analysis](https://www.copetti.org/writings/consoles/game-boy-advance/)

[mgba](https://mgba.io) - MGBA is a modern GBA emulator

[ARM7TDMI Instruction Set Architecture](https://www.ecs.csun.edu/~smirzaei/docs/ece425/arm7tdmi_instruction_set_reference.pdf) - The describes the ARM instruction bit format
## General emulator development resources
[How to Write A Computer Emulator](http://fms.komkon.org/EMUL8/HOWTO.html)

# GBA Development
[GBA Assembly Bare Metal Github](https://github.com/PeterLemon/GBA)

[DevKit Pro](https://devkitpro.org/wiki/Getting_Started) A GBA toolchain

# Testing
We need a way to test our emulator.

Should we code our own test ROM?

# File formats 
## .gba 


## Research
How to handle multiple cpu/gpus.

Should each cpu/gpu be an independent hw thread?

How to handle timing

BIOS
mbga has its own custom BIOS, can we use this?


# Roadmap

# Journal
Install devkitpro gba development tools:
[DevKit Pro Getting Started](https://devkitpro.org/wiki/Getting_Started)

[Devkit Pro Macos Installer](https://github.com/devkitPro/pacman/releases/latest)

# Install gba development tools using devkitpro package manager
sudo dkp-pacman -S gba-dev

# List packages installed:
dkp-pacman -Sl

# Devkitpro installation path 
/opt/devkitpro/

Q: How do we setup paths to run the correct apps?
A: Use /opt/devkitpro/devkitARM/bin/arm-none-eabi-gcc

# Compiling a sample assembly file
```asm
.arm
.text
.global main
main:
	mov r0, #0x4000000
	mov r1, #0x400
	add r1, r1, #3
	str r1, [r0]

	mov r0, #0x6000000
	mov r1, #0xFF
	mov r2, #0x9600
loop1:
	strh r1, [r0], #2
	subs r2, r2, #1
	bne loop1

infin:
	b infin
```

Compile like so
```shell
arm-none-eabi-gcc -mthumb-interwork -spec=gba.specs test_1.S
arm-none-eabi-objcopy -O binary a.out a.gba
```

To see the disassembled output:
```shell
arm-none-eabi-objdump -d | more
```

```objdump-dissassembly
08000000 <__text_start>:
 8000000:       ea00002e        b       80000c0 <rom_header_end>
        ...
 80000b0:       00963130        .word   0x00963130
        ...
 80000bc:       0000f000        .word   0x0000f000

080000c0 <rom_header_end>:
 80000c0:       ea000006        b       80000e0 <start_vector>
 ```

 If we look at the first instruction:
 machinecode: ea00002e

 Lets dig into this instruction since it's an example of ARM peculiarity
 Bits 31-24
 EA = 1110_1010
 Bits 31 to 28 are the Condition bits
 From GBATEK:
 Condition code 0xE (instr suffix AL = ALWAYS, which can be left off). Condition bits determine if the instruction will execute
 based on some condition (e.g. if the carry flag is set). 
 In this case this branch instruction is always executed.

Bits 27-25 are 0b101 meaning its a B/BL/
Bits 24 = 0 so its a B (branch instructions)

Bits 23 - 0 = 0x2e 
Bits 23-0 have a value of 0x2e. But this is an relative to pc offset value, not absolute address. Further more 
its the offset value should be multipled by 4, since its actually offset in # of 32-bit words.
0x2e * 4 = 0xB8. 
But, why does the objdump disassembly say branch to 0x80000c0 and not 0x80000b8? 
Well the branch instruction is actually pc = pc + 8 + 0xB8. Why the + 8? I think 
thats a ARM legacy thing related to the fetch, decode, execute pipeline in 
legacy ARM instructions. By the time this instruction execute's the PC is 
+8 (2 instructions) ahead (i.e. the one in the fetch stage)


Load a.gba in mgba (or another emulator). You should get a red screen

a.out is an ELF file, a.gba is a Cartridge ROM format.

[-] Dump .gba file format


# ARM machine code format.

ARM is a risc processor, it has fixed width instructions 32-bit wide (16 bits for thumb)

There are 3 instruction formats:
    - Data Processing Instructions
    - Memory Instructions
    - Branch Instructions

## Data Processing Instructions
In general the data processing instruction takes:
    - First source register
    - Second source immediate or register , with possible shift
    - Destination register

## Memory Instructions
Memory instructions have 3 operands:
    - Base register
    - Offset that is immediate or shifted register
    - Destination for LDR or Source for STR

## Branch Instructions
Branch instructions take a 24-bit immediate addr offset


# ARMv4 32-bit instruction
Bits 27-26 = instruction type 0b00 = Data Processing, 0b01 = memory , 0b10 = branch


Condition Flags, each instruction also has condition bits that correspond to the conditiion status flags.
These are conditional instructions ,they  only execute conditionally on respective flags. They are good 
for 1 instruction blocks
They can be faster than a branch test, since branches have a delay slot. When the body of the block
gets longer, they branch becomes more efficient. 
