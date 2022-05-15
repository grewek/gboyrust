# Gboyrust (Prototype name)

Gboyrust is a __incomplete__ Gameboy emulator, this project started as a learning project about how cpu works but I
decided that I wanted something more than just another Gameboy emulator so the idea changed into creating an application
in which the user can develop a game.


## Current State

Here you find the current state of all the components of the Gboyrust project.

## CPU

This is the most advanced part of the project. Currently, the emulator can handle almost all opcodes thrown at it,
there is, however, still an opcode left which I do not know how to handle yet. Also the cpu still needs some major
features like cycle counting, interrupt handling and timer support.

## Memory
The memory is currently just a plain flat array and has no switchable banks nor a memory controller. The focus for now is getting the cpu to work right and tackle Memory right after it.

## Controls
Not yet started.

## Video
Not yet started.

## Audio
Not yet started.

## Debugger, Assembler, Compiler, Sprite Editor, Audio Editor
These components all depend on the previous things to work right, so they are not a priority right now. Only
the debugger got started already as I need it to improve my own understanding of the Z80 CPU.