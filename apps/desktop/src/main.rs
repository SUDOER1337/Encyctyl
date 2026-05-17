use eframe::egui;
use encyctyl_core::VaultConfig;
use encyctyl_parser::{parse_note, ParsedNote};
use encyctyl_storage::Db;
use encyctyl_vault::Vault;
use std::path::PathBuf;

struct MyApp {
    vault: Vault,
    db: Db,
    notes: Vec<PathBuf>,
    selected_idx: Option<usize>,
    content: String,
    parsed: Option<ParsedNote>,
    dirty: bool,
}

impl MyApp {
    fn new(vault: Vault, db: Db) -> Self {
        Self {
            vault,
            db,
            notes: Vec::new(),
            selected_idx: None,
            content: String::new(),
            parsed: None,
            dirty: false,
        }
    }

    fn load_note(&mut self, idx: usize) {
        if let Some(path) = self.notes.get(idx) {
            let rel = self.rel_path(path);
            match self.vault.read_note(&rel) {
                Ok(content) => {
                    self.parsed = parse_note(&content).ok();
                    self.content = content;
                    self.dirty = false;
                }
                Err(_) => {
                    self.parsed = None;
                    self.content = String::new();
                    self.dirty = false;
                }
            }
        }
    }

    fn save_current(&mut self) {
        let idx = match self.selected_idx {
            Some(i) => i,
            None => return,
        };
        let path = match self.notes.get(idx) {
            Some(p) => p.clone(),
            None => return,
        };
        let rel = self.rel_path(&path);

        if self.vault.write_note(&rel, &self.content).is_err() {
            return;
        }
        self.parsed = parse_note(&self.content).ok();
        let _ = self.vault.read_and_index_note(&rel, &self.db);
        self.dirty = false;
    }

    fn rel_path(&self, path: &PathBuf) -> PathBuf {
        path.strip_prefix(self.vault.root())
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|_| path.clone())
    }

    fn note_title(&self, idx: usize) -> String {
        let path = match self.notes.get(idx) {
            Some(p) => p,
            None => return String::new(),
        };
        let rel = self.rel_path(path);
        rel.to_string_lossy().to_string()
    }
}

fn main() -> Result<(), eframe::Error> {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());

    let vault = Vault::new(VaultConfig::new(root.into()));
    let db = Db::open_in_memory().expect("failed to open database");

    let mut app = MyApp::new(vault, db);

    let all_notes = app.vault.discover_notes().unwrap_or_default();
    app.notes = all_notes
        .into_iter()
        .filter(|p| !p.to_string_lossy().contains("Example Projects"))
        .collect();

    for note in &app.notes {
        let rel = app.rel_path(note);
        let _ = app.vault.read_and_index_note(&rel, &app.db);
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native("Encyctyl", options, Box::new(|_cc| Ok(Box::new(app))))
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("toolbar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                let title = self
                    .parsed
                    .as_ref()
                    .and_then(|p| p.frontmatter.as_ref())
                    .and_then(|f| f.title.as_deref())
                    .unwrap_or("Encyctyl");
                let dirty_mark = if self.dirty { " *" } else { "" };
                ui.heading(format!("{}{}", title, dirty_mark));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add_enabled(self.dirty, egui::Button::new("Save"))
                        .clicked()
                    {
                        self.save_current();
                    }
                });
            });
        });

        egui::Panel::left("sidebar")
            .resizable(true)
            .default_size(220.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for i in 0..self.notes.len() {
                        let title = self.note_title(i);
                        let selected = self.selected_idx == Some(i);
                        if ui.selectable_label(selected, &title).clicked() {
                            self.selected_idx = Some(i);
                            self.load_note(i);
                        }
                    }
                });
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            if self.selected_idx.is_none() {
                ui.vertical_centered(|ui| {
                    ui.add_space(300.0);
                    ui.label("Select a note from the sidebar");
                });
                return;
            }

            if let Some(parsed) = &self.parsed {
                if let Some(ref fm) = parsed.frontmatter {
                    if let Some(ref title) = fm.title {
                        ui.heading(title);
                    }
                    if !fm.tags.is_empty() {
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Tags:");
                            for tag in &fm.tags {
                                ui.label(format!("#{}", tag));
                            }
                        });
                    }
                    ui.separator();
                }
                if !parsed.wiki_links.is_empty() {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Links:");
                        for link in &parsed.wiki_links {
                            if let Some(ref alias) = link.alias {
                                ui.label(format!("[[{}|{}]]", link.target, alias));
                            } else {
                                ui.label(format!("[[{}]]", link.target));
                            }
                        }
                    });
                    ui.separator();
                }
            }

            let resp = egui::TextEdit::multiline(&mut self.content)
                .font(egui::TextStyle::Monospace)
                .desired_width(f32::INFINITY)
                .show(ui);

            if resp.response.changed() {
                self.dirty = true;
            }

            ui.input_mut(|i| {
                if i.consume_key(egui::Modifiers::CTRL, egui::Key::S) {
                    self.save_current();
                }
            });
        });
    }
}
