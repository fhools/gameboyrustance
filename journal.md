
# Resources 
[GBATEK](http://problemkaputt.de/gbatek.htm)

[Emulation Accuray Speed and Optimization](https://mgba.io/2017/04/30/emulation-accuracy/)

[Gameboy Advanced Architecture - An Analysis](https://www.copetti.org/writings/consoles/game-boy-advance/)

[mgba](https://mgba.io)

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

Load a.gba in mgba (or another emulator). You should get a red screen

a.out is an ELF file, a.gba is a Cartridge ROM format.

[ ] Dump .gba file format
