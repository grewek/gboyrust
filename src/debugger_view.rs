use crate::{
    cpu::register::{RegByte, RegWord},
    debugger::Debugger,
};
use eframe::egui;
use egui::{Align, Color32, RichText};
use egui_extras::{Column, TableBuilder};
use std::collections::HashMap;

use crate::disassembler::AssemblyDesc;

//#[derive(Default)]
pub struct DebuggerView {
    debugger: Debugger,
    disassembly_map: HashMap<u16, usize>,
    disassembly: Vec<([Option<u8>; 3], AssemblyDesc)>,
    font_size: f32,
    selected_index: Option<usize>,
}

impl DebuggerView {
    pub fn new(cc: &eframe::CreationContext<'_>, cartridge: &str) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals {
            dark_mode: true,
            ..egui::Visuals::default()
        });

        let mut view = Self {
            debugger: Debugger::new(),
            disassembly_map: HashMap::new(),
            disassembly: vec![],
            font_size: 18.0,
            selected_index: None,
        };

        view.debugger.load_cartridge(cartridge);
        view.debugger
            .disassemble(&mut view.disassembly, &mut view.disassembly_map);

        view
    }

    fn generate_register_labels(&self, text: &str) -> egui::RichText {
        egui::RichText::new(text)
            .monospace()
            .color(Color32::GOLD)
            .size(self.font_size)
            .underline()
    }

    fn generate_register_value_labels(&self, text: &str) -> egui::RichText {
        egui::RichText::new(text)
            .monospace()
            .color(Color32::YELLOW)
            .size(self.font_size)
            .strong()
    }
}

impl eframe::App for DebuggerView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::S)) {
            self.debugger.step();
        }

        if ctx.input(|i| i.key_pressed(egui::Key::R)) {
            self.debugger.run();
        }

        if self.selected_index.is_some() {
            let offset = self.disassembly[self.selected_index.unwrap()].1.offset;

            if ctx.input(|i| i.key_pressed(egui::Key::B)) {
                self.debugger.toggle_breakpoint(offset);
            }
        }

        if ctx.input(|i| i.key_pressed(egui::Key::D)) {
            self.debugger
                .disassemble(&mut self.disassembly, &mut self.disassembly_map);
        }

        //Rigth panel will hold the current status of the cpu !
        egui::SidePanel::right("cpu_status_pane")
            .min_width(400.0)
            .show(ctx, |ui| {
                egui::Grid::new("Register State").show(ui, |ui| {
                    let register_label_af = self.generate_register_labels("AF:");
                    let register_label_bc = self.generate_register_labels("BC:");
                    let register_label_de = self.generate_register_labels("DE:");
                    let register_label_hl = self.generate_register_labels("HL:");

                    let register_label_a = self.generate_register_labels("A:");
                    let register_label_f = self.generate_register_labels("F:");
                    let register_label_b = self.generate_register_labels("B:");
                    let register_label_c = self.generate_register_labels("C:");
                    let register_label_d = self.generate_register_labels("D:");
                    let register_label_e = self.generate_register_labels("E:");
                    let register_label_h = self.generate_register_labels("H:");
                    let register_label_l = self.generate_register_labels("L:");

                    let register_label_sp = self.generate_register_labels("SP:");
                    let register_label_pc = self.generate_register_labels("PC:");
                    let machine_cycles = self.generate_register_labels("CYCLES:");
                    let t_cycles = self.generate_register_labels("T-CYCLES:");
                    let div_timer = self.generate_register_labels("DIV TIMER:");
                    let custom_timer = self.generate_register_labels("CUSTOM TIMER:");
                    let tac_control = self.generate_register_labels("TICK RATE:");
                    let custom_timer_reset = self.generate_register_labels("RESET VALUE:");

                    ui.label(register_label_af);
                    ui.label(self.generate_register_value_labels(
                        &self.debugger.get_register_word(RegWord::Af),
                    ));

                    ui.label("[ ");
                    ui.label(register_label_a);
                    ui.label(self.generate_register_value_labels(
                        &self.debugger.get_register_byte(RegByte::A),
                    ));
                    ui.label(register_label_f);
                    ui.label(self.generate_register_value_labels(
                        &self.debugger.get_register_byte(RegByte::F),
                    ));
                    ui.label(" ]");
                    ui.end_row();

                    ui.label(register_label_bc);
                    ui.label(self.generate_register_value_labels(
                        &self.debugger.get_register_word(RegWord::Bc),
                    ));

                    ui.label("[ ");
                    ui.label(register_label_b);
                    ui.label(self.generate_register_value_labels(
                        &self.debugger.get_register_byte(RegByte::B),
                    ));
                    ui.label(register_label_c);
                    ui.label(self.generate_register_value_labels(
                        &self.debugger.get_register_byte(RegByte::C),
                    ));
                    ui.label(" ]");
                    ui.end_row();

                    ui.label(register_label_de);
                    ui.label(self.generate_register_value_labels(
                        &self.debugger.get_register_word(RegWord::De),
                    ));

                    ui.label("[ ");
                    ui.label(register_label_d);
                    ui.label(self.generate_register_value_labels(
                        &self.debugger.get_register_byte(RegByte::D),
                    ));
                    ui.label(register_label_e);
                    ui.label(self.generate_register_value_labels(
                        &self.debugger.get_register_byte(RegByte::E),
                    ));
                    ui.label(" ]");
                    ui.end_row();

                    ui.label(register_label_hl);
                    ui.label(self.generate_register_value_labels(
                        &self.debugger.get_register_word(RegWord::Hl),
                    ));

                    ui.label("[ ");
                    ui.label(register_label_h);
                    ui.label(self.generate_register_value_labels(
                        &self.debugger.get_register_byte(RegByte::H),
                    ));
                    ui.label(register_label_l);
                    ui.label(self.generate_register_value_labels(
                        &self.debugger.get_register_byte(RegByte::L),
                    ));
                    ui.label(" ]");
                    ui.end_row();

                    ui.label(register_label_sp);
                    ui.label(self.generate_register_value_labels(&self.debugger.get_sp_string()));
                    ui.end_row();

                    ui.label(register_label_pc);
                    ui.label(self.generate_register_value_labels(&self.debugger.get_pc_string()));
                    ui.end_row();
                    ui.end_row();

                    ui.label(machine_cycles);
                    let cycles = self.debugger.get_machine_cycles();
                    let t_cycles_value = cycles * 4;
                    let cycles = format!("{}", &cycles);
                    let t_cycles_str = format!("{}", &t_cycles_value);
                    ui.label(self.generate_register_value_labels(&cycles));
                    ui.end_row();

                    ui.label(t_cycles);
                    ui.label(self.generate_register_value_labels(&t_cycles_str));
                    ui.end_row();
                    ui.end_row();

                    ui.label(div_timer);
                    let timer = format!("{}", &self.debugger.get_div_timer());
                    ui.label(self.generate_register_value_labels(&timer));
                    ui.end_row();
                    ui.end_row();

                    ui.label(custom_timer);
                    let timer = &self.debugger.get_custom_timer();
                    ui.label(self.generate_register_value_labels(&timer));
                    ui.end_row();
                    ui.label(tac_control);
                    let tac_value = &self.debugger.get_custom_timer_tick_rate();
                    ui.label(self.generate_register_labels(&tac_value));
                    ui.end_row();

                    ui.label(custom_timer_reset);
                    let timer_reset_value = &self.debugger.get_timer_reset();
                    ui.label(self.generate_register_value_labels(&timer_reset_value));
                });
            });

        egui::TopBottomPanel::bottom("call stack")
            .min_height(240.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("stack").show(ui, |ui| {
                        for data in self.debugger.stack_data() {
                            let data_fmt = if data.0 {
                                format!("--> {:04X}: {:04X}", data.1, data.2)
                            } else {
                                format!("    {:04X}: {:04X}", data.1, data.2)
                            };

                            let data_label = self.generate_register_value_labels(&data_fmt);

                            ui.label(data_label);
                            ui.end_row();
                        }
                    });
                });
            });

        //Central panel contains the disassembly
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut table = TableBuilder::new(ui)
                .striped(true)
                .max_scroll_height(1280.0)
                .column(Column::initial(80.0))
                .column(Column::initial(150.0))
                .column(Column::initial(150.0))
                .column(Column::initial(150.0))
                .column(Column::remainder());
            if ctx.input(|i| i.key_pressed(egui::Key::Enter)) && self.selected_index.is_some() {
                let current_op = self.disassembly[self.selected_index.unwrap()].1;

                if let Some(address) = current_op.follow() {
                    table = table.scroll_to_row(
                        self.disassembly_map[&(address as u16)],
                        Some(Align::Center),
                    );
                }
            }

            if ctx.input(|i| i.key_pressed(egui::Key::Backspace)) {
                table = table.scroll_to_row(
                    self.disassembly_map[&(self.debugger.get_program_counter() as u16)],
                    Some(Align::Center),
                );
            }

            table = table.sense(egui::Sense::click());
            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Offset");
                    });

                    header.col(|ui| {
                        ui.heading("Hex Dump");
                    });

                    header.col(|ui| {
                        ui.heading("Opcode Mnemoic");
                    });

                    //TODO: These columns don't feel "right" it's probably better to show the
                    //      disassembly in a complete chunk instead of seperating it. I need to
                    //      think about that more and probably make some visual tests
                    header.col(|ui| {
                        ui.heading("Dest");
                    });

                    header.col(|ui| {
                        ui.heading("Src");
                    });
                })
                .body(|body| {
                    let row_height = 20.0;
                    let row_count = self.disassembly.len();

                    body.rows(row_height, row_count, |mut row| {
                        let row_index = row.index();
                        let disassembly = &self.disassembly[row_index];

                        let mut col = Color32::GRAY;

                        if let Some(index) = self.selected_index {
                            if index == row_index {
                                col = Color32::GOLD;
                            }
                        }

                        if self.debugger.is_registered_breakpoint(disassembly.1.offset) {
                            col = Color32::RED;
                        }

                        if (disassembly.1.offset as usize) == self.debugger.get_program_counter() {
                            col = Color32::GREEN;
                        }

                        let offset_text = format!("{:04X}", disassembly.1.offset);
                        let offset_label = RichText::new(&offset_text)
                            .color(col)
                            .size(self.font_size)
                            .monospace();

                        let disassembly_text = format!("{}", disassembly.1.opcode);
                        let dest_text = format!("{}", disassembly.1.dest);
                        let src_text = format!("{}", disassembly.1.src);

                        let disassembly_label = RichText::new(&disassembly_text)
                            .color(col)
                            .size(self.font_size)
                            .monospace();
                        let dest_label = RichText::new(dest_text)
                            .color(col)
                            .size(self.font_size)
                            .monospace();
                        let src_label = RichText::new(src_text)
                            .color(col)
                            .size(self.font_size)
                            .monospace();

                        row.col(|ui| {
                            ui.label(offset_label);
                        });

                        let dump = disassembly.0;
                        let hexdump = match dump {
                            [Some(op), None, None] => format!("{:02X}", op),
                            [Some(op), Some(arg_a), None] => format!("{:02X} {:02X}", op, arg_a),
                            [Some(op), Some(arg_a), Some(arg_b)] => {
                                format!("{:02X} {:02X} {:02X}", op, arg_a, arg_b)
                            }
                            _ => panic!("malformed hexdump pattern"),
                        };

                        let hexdump_label = RichText::new(&hexdump)
                            .color(col)
                            .size(self.font_size)
                            .monospace();
                        row.col(|ui| {
                            ui.label(hexdump_label);
                        });
                        row.col(|ui| {
                            ui.label(disassembly_label);
                        });
                        row.col(|ui| {
                            ui.label(dest_label);
                        });
                        row.col(|ui| {
                            ui.label(src_label);
                        });

                        if row.response().clicked() {
                            self.selected_index = Some(row_index);
                        }
                    });
                });
        });
    }
}
