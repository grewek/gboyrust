mod cpu;
mod debugger;
mod disassembler;
mod memory;

use std::env::args;
use std::fs::File;
use std::io::{stdout, Read, Write};

use cpu::Cpu;
use crossterm::terminal::disable_raw_mode;
use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    style::{self, Color, Print},
    terminal::{self, enable_raw_mode, size},
    QueueableCommand, Result,
};
use debugger::{Debugger, DebuggerCommand};
use disassembler::AssemblyDesc;

fn main() -> Result<()> {
    //let path = "test_roms/mooneye-tests/acceptance/bits/reg_f.gb";
    let args: Vec<String> = args().collect();

    let path = "test_roms/cpu_instrs/individual/06-ld r,r.gb";
    //let path = "test_roms/dmg_boot.bin";

    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

    let mut mem = memory::Memory::default();
    mem.load_cartridge(&buffer);

    if args.len() > 1 {
        let mut stdout = stdout();
        let (_, con_height) = size()?;

        stdout.queue(terminal::Clear(terminal::ClearType::All))?;

        enable_raw_mode()?;

        let mut debugger = debugger::Debugger::new(mem);
        loop {
            let disassembly = debugger.disassemble_region(con_height - 1);

            stdout.queue(terminal::Clear(terminal::ClearType::All))?;

            for (i, line) in disassembly.iter().enumerate() {
                let dump = debugger.create_dump(line.offset, line.size);

                if debugger.is_active_breakpoint(line.offset) {
                    print_assembly_line(&line, &dump, Color::Green, 5, i as u16)?;
                } else if debugger.is_executed_next(line.offset) {
                    print_assembly_line(&line, &dump, Color::Red, 5, i as u16)?;
                } else {
                    print_assembly_line(&line, &dump, Color::Grey, 5, i as u16)?;
                }

                print_cpu_state(&debugger.get_cpu_state())?;
                let stack = debugger.get_stack_view(10);
                print_stack_view(&stack)?;
                print_flags(&debugger.get_flags())?;
            }

            stdout.flush()?;

            match event::read()? {
                event::Event::Key(p) => process_key_input(&mut debugger, con_height, p),
                event::Event::Mouse(_) => {}
                event::Event::Resize(_, _) => {}
            }
            print_cpu_state(&debugger.get_cpu_state())?;
            stdout.flush()?;
        }
    } else {
        let mut cpu = Cpu::default();

        loop {
            cpu.cycle(&mut mem);
        }

        Ok(())
    }
}

fn dump_to_string(dump: &[Option<u8>; 3]) -> String {
    match dump {
        [Some(op), None, None] => format!("{:02X}", op),
        [Some(op), Some(lo), None] => format!("{:02X} {:02X}", op, lo),
        [Some(op), Some(lo), Some(hi)] => format!("{:02X} {:02X} {:02X}", op, lo, hi),
        _ => panic!("Malformed byte dump !"),
    }
}

fn print_flags(flags: &str) -> Result<()> {
    stdout().queue(cursor::MoveTo(60, 7))?.queue(Print(flags))?;

    Ok(())
}
fn print_cpu_state(state: &[String; 6]) -> Result<()> {
    stdout()
        .queue(cursor::MoveTo(60, 0))?
        .queue(Print(&state[0]))?
        .queue(cursor::MoveTo(60, 1))?
        .queue(Print(&state[1]))?
        .queue(cursor::MoveTo(60, 2))?
        .queue(Print(&state[2]))?
        .queue(cursor::MoveTo(60, 3))?
        .queue(Print(&state[3]))?
        .queue(cursor::MoveTo(60, 4))?
        .queue(Print(&state[4]))?
        .queue(cursor::MoveTo(60, 5))?
        .queue(Print(&state[5]))?;

    Ok(())
}

fn print_assembly_line(
    asm_info: &AssemblyDesc,
    dump: &[Option<u8>; 3],
    color: Color,
    gap: usize,
    line_index: u16,
) -> Result<()> {
    let offset = format!("{:04X}", &asm_info.offset);
    let dump = dump_to_string(dump);
    let disassembly_distance = 8 - dump.len();

    let mut stdout = stdout();
    stdout
        .queue(style::SetForegroundColor(color))?
        .queue(cursor::MoveTo(0, line_index))?
        .queue(Print(&offset))?
        .queue(cursor::MoveTo((offset.len() + gap) as u16, line_index))?
        .queue(Print(&dump))?
        .queue(cursor::MoveTo(
            (offset.len() + disassembly_distance + gap + dump.len() + gap) as u16,
            line_index,
        ))?
        .queue(Print(asm_info.to_string()))?
        .queue(style::SetForegroundColor(Color::Grey))?;
    Ok(())
}

fn print_stack_view(view: &[String]) -> Result<()> {
    for (i, stack_str) in view.iter().enumerate() {
        stdout()
            .queue(cursor::MoveTo(60, 10 + i as u16))?
            .queue(Print(stack_str))?;
    }
    Ok(())
}

fn process_key_input(debugger: &mut Debugger, con_height: u16, key_event: KeyEvent) {
    if key_event.modifiers == KeyModifiers::CONTROL && key_event.code == KeyCode::Char('c') {
        disable_raw_mode();
        std::process::exit(0);
    }

    if key_event.code == KeyCode::Char('n') {
        debugger.send_command(DebuggerCommand::Step);
    }

    if key_event.code == KeyCode::Char('r') {
        debugger.send_command(DebuggerCommand::Run);
    }

    if key_event.code == KeyCode::Char('o') {
        debugger.send_command(DebuggerCommand::StepOver);
    }

    let mut buffer = String::new();
    if key_event.code == KeyCode::Char(':') {
        loop {
            match event::read().expect("ERROOR") {
                event::Event::Key(k) => {
                    if k.code == KeyCode::Enter {
                        process_line_and_send(debugger, &buffer);
                        break;
                    }

                    match k.code {
                        KeyCode::Char(c) => buffer.push(c),
                        KeyCode::Backspace => {
                            let _ = buffer.remove(buffer.len() - 1);
                        }
                        _ => continue,
                    }
                }
                event::Event::Mouse(_) => {}
                event::Event::Resize(_, _) => {}
            }

            stdout().flush().unwrap();
            let output = format!(": {}", &buffer);
            stdout()
                .queue(cursor::MoveTo(0, con_height - 1))
                .unwrap()
                .queue(terminal::Clear(terminal::ClearType::UntilNewLine))
                .unwrap()
                .queue(Print(&output))
                .unwrap();

            stdout().flush().unwrap();
        }
    }
}

fn process_line_and_send(debugger: &mut Debugger, buffer: &str) {
    let parts: Vec<String> = buffer.split_whitespace().map(|s| s.to_string()).collect();

    let operation = match parts[0].trim() {
        //TODO: This will crash if the user doesn't supply a second argument thats in the correct format and can be parsed
        //      into a u16
        "b" => DebuggerCommand::ToggleBreakpoint(u16::from_str_radix(parts[1].trim(), 16).unwrap()),
        "r" => DebuggerCommand::Run,
        "n" => DebuggerCommand::Step,
        "o" => DebuggerCommand::StepOver,

        _ => DebuggerCommand::Ignore,
    };

    debugger.send_command(operation);
}
