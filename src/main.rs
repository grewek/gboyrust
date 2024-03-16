mod assembler;
mod cpu;
mod debugger;
mod debugger_view;
mod disassembler;
mod memory;

use std::env;
use std::fs::File;
use std::io::Read;

use debugger_view::DebuggerView;

fn main() {
    let mut args = env::args();

    if args.len() < 2 {
        println!("Usage: gboyrust <rompath>");
        return;
    }

    let rom_path = args.nth(1).unwrap();

    let native_options = eframe::NativeOptions {
        ..eframe::NativeOptions::default()
    };
    //let path = "test_roms/mooneye-tests/acceptance/bits/reg_f.gb";
    //let path = "test_roms/cpu_instrs/individual/01-special.gb"; // PASSED (Not quite sure the daa
    //opcode seems to be tricky)

    //let path = "test_roms/cpu_instrs/individual/02-interrupts.gb";
    //let path = "test_roms/cpu_instrs/individual/03-op sp,hl.gb"; //PASSED !
    //let path = "test_roms/cpu_instrs/individual/04-op r,imm.gb"; //PASSED !
    //let path = "test_roms/cpu_instrs/individual/05-op rp.gb"; //PASSED !
    //let path = "test_roms/cpu_instrs/individual/06-ld r,r.gb"; //PASSED !
    //let path = "test_roms/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb"; //PASSED !
    //let path = "test_roms/cpu_instrs/individual/08-misc instrs.gb"; //PASSED !
    //let path = "test_roms/cpu_instrs/individual/09-op r,r.gb"; //PASSED !
    //let path = "test_roms/cpu_instrs/individual/10-bit ops.gb"; //PASSED !
    //let path = "test_roms/cpu_instrs/individual/11-op a,(hl).gb"; //PASSED !
    //let path = "test_roms/dmg_boot.bin";

    //let mut file = File::open(path).unwrap();
    //let mut buffer = Vec::new();

    //file.read_to_end(&mut buffer).unwrap();

    //let mut mem = memory::Memory::default();
    //mem.load_cartridge(&buffer);

    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(move |cc| {
            //let mut file = File::open(path).unwrap();
            //let mut buffer = Vec::new();

            //file.read_to_end(&mut buffer).unwrap();
            Box::new(DebuggerView::new(cc, &rom_path))
        }),
    );
    //let mut cpu = Cpu::default();

    //loop {
    //cpu.cycle(&mut mem);
    //}
}
