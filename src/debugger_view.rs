use crate::debugger::Debugger;
use crate::debugger::DisassemblyCache;
use crate::debugger::MemoryLayout;
use eframe::egui;
use egui_extras::{Size, TableBuilder};
//#[derive(Default)]
pub struct DebuggerView {
    selected: MemoryLayout,
    debugger: Debugger,
    font_size: f32,
}

impl DebuggerView {
    pub fn new(_cc: &eframe::CreationContext<'_>, cartridge: &str) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        _cc.egui_ctx.set_visuals(egui::Visuals {
            dark_mode: true,
            ..egui::Visuals::default()
        });

        let mut view = Self {
            selected: MemoryLayout::RomBankZero,
            debugger: Debugger::new(),
            font_size: 24.0,
        };

        view.debugger.load_cartridge(cartridge);

        view
    }
}

impl eframe::App for DebuggerView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.debugger.disassemble(&self.selected);
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            egui::ComboBox::from_label("Region to Disassemble")
                .selected_text(format!("{:?}", self.selected))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.selected,
                        MemoryLayout::RomBankZero,
                        "Rom Bank Zero",
                    );
                    ui.selectable_value(
                        &mut self.selected,
                        MemoryLayout::RomBankOne,
                        "Rom Bank One",
                    );
                    ui.selectable_value(
                        &mut self.selected,
                        MemoryLayout::ExternalMemory,
                        "External Memory",
                    );
                    ui.selectable_value(
                        &mut self.selected,
                        MemoryLayout::WorkingRamZero,
                        "WRam Zero",
                    );
                    ui.selectable_value(
                        &mut self.selected,
                        MemoryLayout::WorkingRamOne,
                        "WRam One",
                    );
                    ui.selectable_value(&mut self.selected, MemoryLayout::MirrorRam, "Mirror Ram");
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            let current_position = self.debugger.get_program_counter();
            dbg!(current_position);
            let disassembly = &self.debugger.get_disassembly(&self.selected);
            let row_height = self.font_size;
            let total_rows = disassembly.len();
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show_rows(ui, row_height, total_rows, |ui, row_range| {
                    egui::Grid::new("disassembly")
                        .start_row(current_position) //TODO: Figure out why this function is not working as expected !
                        .show(ui, |ui| {
                            for asm in &disassembly[row_range] {
                                let offset_text = egui::RichText::new(&asm.offset)
                                    .monospace()
                                    .size(self.font_size);
                                ui.label(offset_text);

                                let hexdump_text = egui::RichText::new(&asm.hexdump)
                                    .monospace()
                                    .size(self.font_size);
                                ui.label(hexdump_text);

                                let opcode_text = egui::RichText::new(&asm.opcode)
                                    .monospace()
                                    .size(self.font_size);
                                ui.label(opcode_text);
                                ui.separator();
                                let argument_text = egui::RichText::new(&asm.argument)
                                    .monospace()
                                    .size(self.font_size);
                                ui.label(argument_text);
                                ui.end_row();
                            }
                        })
                });
        });
    }
}
