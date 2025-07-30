use crate::app::ZenView;
use std::path::PathBuf;
use crate::ui::theme::ZenTheme;
use crate::ui::tree::FileExplorer;

pub struct CodeEditor {
    language: String,
    pub code: String,
    pub selected_file: Option<PathBuf>,
    file_explorer: FileExplorer,
    pub theme: ZenTheme,
    pub available_themes: Vec<ZenTheme>,
    pub selected_theme_index: usize,
}

impl Default for CodeEditor {
    fn default() -> Self {
        let themes = ZenTheme::load_available_themes();
        Self {
            language: "rs".into(),
            code: "".into(),
            selected_file: None,
            file_explorer: FileExplorer::default(),
            theme: ZenTheme::default(),
            available_themes: themes,
            selected_theme_index: 0,
        }
    }
}

impl CodeEditor {
    pub fn open_project(&mut self, path: PathBuf) {
        self.file_explorer.open_project(path);
    }

    pub fn set_theme(&mut self, theme: ZenTheme) {
        self.theme = theme;
        if let Some(index) = self.available_themes.iter().position(|t| t.name == self.theme.name) {
            self.selected_theme_index = index;
        }
    }

    pub(crate) fn load_file(&mut self, path: &PathBuf) {
        if let Ok(content) = std::fs::read_to_string(path) {
            self.code = content;
            self.selected_file = Some(path.clone());
            if let Some(name) = path.file_name() {
                if name.to_string_lossy().eq_ignore_ascii_case("CMakeLists.txt") {
                    self.language = "c".into();
                } else if let Some(ext) = path.extension() {
                    self.language = ext.to_string_lossy().to_string();
                }
            }
        }
    }
}

impl ZenView for CodeEditor {
    fn ui(&mut self, ui: &mut egui::Ui) {
        self.theme.apply_to_context(ui.ctx());

        egui::SidePanel::left("file_tree")
            .resizable(true)
            .default_width(200.0)
            .width_range(150.0..=400.0)
            .show_animated_inside(ui, self.file_explorer.root.is_some(), |ui| {
                self.file_explorer.render(ui);
            });

        if let Some(path) = self.file_explorer.take_pending_file() {
            self.load_file(&path);
        }

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let syntax_theme = if self.theme.name.contains("Light") {
                egui_extras::syntax_highlighting::CodeTheme::light(self.theme.typography.code_font_size)
            } else {
                egui_extras::syntax_highlighting::CodeTheme::dark(self.theme.typography.code_font_size)
            };

            let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, _wrap_width: f32| {
                let mut layout_job = egui_extras::syntax_highlighting::highlight(
                    ui.ctx(),
                    ui.style(),
                    &syntax_theme,
                    buf.as_str(),
                    &self.language,
                );
                layout_job.wrap.max_width = f32::INFINITY;
                ui.fonts(|f| f.layout_job(layout_job))
            };

            let line_count = self.code.lines().count().max(1);
            let line_height = ui.text_style_height(&egui::TextStyle::Monospace);
            let content_height = line_count as f32 * line_height;

            let max_line_width = self.code.lines()
                .map(|line| {
                    ui.fonts(|f| f.layout_no_wrap(
                        line.to_string(),
                        egui::FontId::monospace(self.theme.typography.code_font_size),
                        egui::Color32::WHITE
                    ).size().x)
                })
                .fold(0.0, f32::max);

            #[allow(deprecated)]
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let frame = egui::Frame::new()
                        .fill(self.theme.editor_background())
                        .rounding(egui::Rounding::same(8))
                        .inner_margin(egui::Margin::same(self.theme.spacing.panel_margin));

                    frame.show(ui, |ui| {
                        let min_width = max_line_width.max(ui.available_width());
                        let min_height = content_height.max(ui.available_height());

                        let response = ui.add_sized(
                            [min_width, min_height],
                            egui::TextEdit::multiline(&mut self.code)
                                .font(egui::TextStyle::Monospace)
                                .code_editor()
                                .desired_rows(line_count.max(20))
                                .desired_width(min_width)
                                .layouter(&mut layouter),
                        );

                        if !response.has_focus() {
                            response.request_focus();
                        }
                    });
                });
        });
    }
}