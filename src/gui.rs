use crate::Space;
use crate::errors::IOParseError;

/// A Graphical User Interface
#[derive(Debug, Default)]
pub struct Gui {
    space: Space,
}

impl Gui {
    /// Load the given path into the [Space]
    pub fn load<P>(&mut self, path: P) -> std::result::Result<(), IOParseError>
    where
        P: AsRef<std::path::Path>,
    {
        self.space = Space::try_from(path.as_ref())?;
        Ok(())
    }

    /// Launch the GUI.
    pub fn run(self) -> std::io::Result<()> {
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "🤖BEFUNGATON🤖",
            options,
            Box::new(|_cc| Ok(Box::new(self))),
        )
        .map_err(std::io::Error::other)
    }
}

impl eframe::App for Gui {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        use eframe::egui;

        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }

        ui.vertical_centered(|ui| {
            ui.colored_label(egui::Color32::from_rgb(255, 0, 255), "🤖BEFUNGATON🤖");
        });
        ui.separator();
        ui.monospace(self.space.to_string());
    }
}
