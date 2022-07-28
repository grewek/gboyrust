use crate::{debugger::Debugger, cpu::register::{RegWord, RegByte}};
use eframe::egui;
use egui::Color32;
//#[derive(Default)]
pub struct DebuggerView {
    debugger: Debugger,
    disassembly: Vec<(usize, String)>,
    font_size: f32,
}

impl DebuggerView {
    pub fn new(cc: &eframe::CreationContext<'_>, cartridge: &str) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals {
            dark_mode: true,
            ..egui::Visuals::default()
        });

        let mut view = Self {
            debugger: Debugger::new(),
            disassembly: vec![],
            font_size: 18.0,
        };

        view.debugger.load_cartridge(cartridge);
        view.debugger.disassemble(&mut view.disassembly);
        
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
        if ctx.input().key_pressed(egui::Key::N) {
            self.debugger.step()
        }

        if ctx.input().key_pressed(egui::Key::D) {
            self.debugger.disassemble(&mut self.disassembly);
        }


        //Rigth panel will hold the current status of the cpu !
        egui::SidePanel::right("cpu_status_pane").min_width(400.0).show(ctx, |ui| {
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

                ui.label(register_label_af);
                ui.label(self.generate_register_value_labels(&self.debugger.get_register_word(RegWord::Af)));
                
                ui.label("[ ");
                ui.label(register_label_a);
                ui.label(self.generate_register_value_labels(&self.debugger.get_register_byte(RegByte::A)));
                ui.label(register_label_f);
                ui.label(self.generate_register_value_labels(&self.debugger.get_register_byte(RegByte::F)));
                ui.label(" ]");
                ui.end_row();
                
                
                ui.label(register_label_bc);
                ui.label(self.generate_register_value_labels(&self.debugger.get_register_word(RegWord::Bc)));
                
                ui.label("[ ");
                ui.label(register_label_b);
                ui.label(self.generate_register_value_labels(&self.debugger.get_register_byte(RegByte::B)));
                ui.label(register_label_c);
                ui.label(self.generate_register_value_labels(&self.debugger.get_register_byte(RegByte::C)));
                ui.label(" ]");
                ui.end_row();

                ui.label(register_label_de);
                ui.label(self.generate_register_value_labels(&self.debugger.get_register_word(RegWord::De)));

                ui.label("[ ");
                ui.label(register_label_d);
                ui.label(self.generate_register_value_labels(&self.debugger.get_register_byte(RegByte::D)));
                ui.label(register_label_e);
                ui.label(self.generate_register_value_labels(&self.debugger.get_register_byte(RegByte::E)));
                ui.label(" ]");
                ui.end_row();

                ui.label(register_label_hl);
                ui.label(self.generate_register_value_labels(&self.debugger.get_register_word(RegWord::Hl)));
                
                ui.label("[ ");
                ui.label(register_label_h);
                ui.label(self.generate_register_value_labels(&self.debugger.get_register_byte(RegByte::H)));
                ui.label(register_label_l);
                ui.label(self.generate_register_value_labels(&self.debugger.get_register_byte(RegByte::L)));
                ui.label(" ]");
                ui.end_row();

                ui.label(register_label_sp);
                ui.label(self.generate_register_value_labels(&self.debugger.get_sp_string()));
                ui.end_row();

                ui.label(register_label_pc);
                ui.label(self.generate_register_value_labels(&self.debugger.get_pc_string()));
                ui.end_row();
            });
        });

        //Central panel contains the disassembly
        egui::CentralPanel::default().show(ctx, |ui| {
            let current_position = self.debugger.get_program_counter();

            let row_height = self.font_size;
            let total_rows = self.disassembly.len();

            

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show_rows(ui, row_height, total_rows, |ui, row_range| {
                    egui::Grid::new("disassembly")
                        .show(ui, |ui| {
                            for (offset, instruction) in &self.disassembly[row_range] {
                                let line_color = if *offset == current_position {
                                    Color32::GREEN
                                } else {
                                    Color32::WHITE
                                };

                                let disassembly = egui::RichText::new(instruction)
                                    .monospace()
                                    .color(line_color)
                                    .size(self.font_size);
                                
                                ui.label(disassembly);
                                ui.end_row();
                            }
                        })
                });
        });
    }
}
