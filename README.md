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

# Debugger Short Manual
To start into the debugger you need to load up a rom file i.e.:

via cargo:
`
$ cargo run path/to/rom/testrom.gb
`

If you did it right the debugger gui will appear.

On the left side you have the Disassembly view it shows you all the instructions in memory. A line that is highlighted with a green color is indicating the instruction that will be executed in the next cycle of the emulation. A active breakpoint is indicated by a red instruction, if hit the execution will halt on that line. Lines highlighted in a golden color are indicating instruction you are currently hovering over with your mouse pointer.

To the right of the disassembly you have the register states of the cpu.

On to the controls of the debugger: 

* Press __s__ to step to the next instruction.
* Press __r__ to start running. (__WARNING__ This will hang in an infinite loop if no breakpoint is set if that happens you need to kill the whole process with ^C(Ctrl+C). I hope to fix this soon.)
* Hover over a disassembly line with your mouse and press __b__ to set or unset a breakpoint 
* Hover over a JP or CALL instruction and press __enter__ to scroll the disassembly view to the target address.
* Press __backspace__ to scroll the view to the current position of the program counter.
* Press __d__ to disassemble the whole memory again, this is necessary if the program writes into ram at runtime.

__NOTE__: The debugger is in development so things can change quite rapidly.

