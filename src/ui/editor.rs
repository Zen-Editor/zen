use crate::app::ZenView;
use crate::ui::theme::ZenTheme;
use crate::ui::tree::FileExplorer;
use std::path::PathBuf;

pub struct CodeEditor {
    language: String,
    pub code: String,
    pub selected_file: Option<PathBuf>,
    file_explorer: FileExplorer,
    pub theme: ZenTheme,
    pub available_themes: Vec<ZenTheme>,
    pub selected_theme_index: usize,
    show_line_numbers: bool,
    document_version: i32,
    cached_line_height: Option<f32>,
    cached_max_line_width: Option<f32>,
    cached_layout_job: Option<egui::text::LayoutJob>,
    cache_version: i32,
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
            show_line_numbers: true,
            document_version: 0,
            cached_line_height: None,
            cached_max_line_width: None,
            cached_layout_job: None,
            cache_version: -1,
        }
    }
}

impl CodeEditor {
    pub fn open_project(&mut self, path: PathBuf) {
        self.file_explorer.open_project(path.clone());
    }

    pub fn set_theme(&mut self, theme: ZenTheme) {
        self.theme = theme;
        if let Some(index) = self.available_themes.iter().position(|t| t.name == self.theme.name) {
            self.selected_theme_index = index;
        }

        self.cached_layout_job = None;
        self.cached_line_height = None;
    }

    pub fn load_file(&mut self, path: &PathBuf) {
        if let Ok(content) = std::fs::read_to_string(path) {
            self.code = content;
            self.selected_file = Some(path.clone());
            self.document_version += 1;

            self.invalidate_caches();

            if let Some(name) = path.file_name() {
                if name.to_string_lossy().eq_ignore_ascii_case("CMakeLists.txt") {
                    self.language = "c".into();
                } else if let Some(ext) = path.extension() {
                    self.language = ext.to_string_lossy().to_string();
                }
            }
        }
    }

    fn invalidate_caches(&mut self) {
        self.cached_line_height = None;
        self.cached_max_line_width = None;
        self.cached_layout_job = None;
        self.cache_version = self.document_version;
    }

    fn get_line_height(&mut self, ui: &egui::Ui) -> f32 {
        if self.cached_line_height.is_none() || self.cache_version != self.document_version {
            self.cached_line_height = Some(
                ui.fonts(|f| f.row_height(&egui::FontId::monospace(self.theme.typography.code_font_size)))
            );
            self.cache_version = self.document_version;
        }
        self.cached_line_height.unwrap()
    }

    fn get_max_line_width(&mut self, ui: &egui::Ui) -> f32 {
        if self.cached_max_line_width.is_none() || self.cache_version != self.document_version {
            let max_lines_to_check = 1000;
            let lines: Vec<&str> = self.code.lines().take(max_lines_to_check).collect();

            self.cached_max_line_width = Some(
                lines.iter()
                    .map(|line| {
                        ui.fonts(|f| f.layout_no_wrap(
                            line.to_string(),
                            egui::FontId::monospace(self.theme.typography.code_font_size),
                            egui::Color32::WHITE
                        ).size().x)
                    })
                    .fold(0.0, f32::max)
                    .max(800.0)
            );
            self.cache_version = self.document_version;
        }
        self.cached_max_line_width.unwrap()
    }

    fn get_highlighted_layout(&mut self, text: &str) -> egui::text::LayoutJob {
        if self.cached_layout_job.is_none() || self.cache_version != self.document_version {
            let mut layout_job = self.theme.highlight_code(text, &self.language);
            layout_job.wrap.max_width = f32::INFINITY;
            self.cached_layout_job = Some(layout_job);
            self.cache_version = self.document_version;
        }
        self.cached_layout_job.as_ref().unwrap().clone()
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
            self.render_editor_panel(ui);
        });
    }
}

impl CodeEditor {
    fn render_editor_panel(&mut self, ui: &mut egui::Ui) {
        let line_count = self.code.lines().count().max(1);
        let line_height = self.get_line_height(ui);
        let line_number_width = self.calculate_line_number_width(ui, line_count);

        let frame = self.create_editor_frame();

        frame.show(ui, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    self.render_editor_content(ui, line_count, line_height, line_number_width);
                });
        });
    }

    fn calculate_line_number_width(&self, ui: &egui::Ui, line_count: usize) -> f32 {
        let max_line_digits = line_count.to_string().len();
        ui.fonts(|f| {
            f.layout_no_wrap(
                "9".repeat(max_line_digits + 1),
                egui::FontId::monospace(self.theme.typography.code_font_size),
                self.theme.text_color(),
            ).size().x
        }) + if self.show_line_numbers { 16.0 } else { 0.0 }
    }

    #[allow(deprecated)]
    fn create_editor_frame(&self) -> egui::Frame {
        egui::Frame::new()
            .fill(self.theme.editor_background())
            .rounding(egui::Rounding::same(8))
            .inner_margin(egui::Margin::same(self.theme.spacing.panel_margin))
    }

    fn render_editor_content(&mut self, ui: &mut egui::Ui, line_count: usize, line_height: f32, line_number_width: f32) {
        ui.horizontal_top(|ui| {
            if self.show_line_numbers {
                self.render_line_numbers(ui, line_count, line_height, line_number_width);
            }

            let (editor_width, editor_height) = self.calculate_editor_dimensions(ui, line_count, line_height);
            self.render_text_editor(ui, editor_width, editor_height);
        });
    }

    fn render_line_numbers(&self, ui: &mut egui::Ui, line_count: usize, line_height: f32, line_number_width: f32) {
        ui.allocate_ui_with_layout(
            egui::vec2(line_number_width, line_height * line_count as f32),
            egui::Layout::top_down(egui::Align::RIGHT),
            |ui| {
                self.draw_line_number_background(ui);
                self.configure_line_number_spacing(ui);
                self.draw_line_numbers(ui, line_count, line_height, line_number_width);
            }
        );
    }

    #[allow(deprecated)]
    fn draw_line_number_background(&self, ui: &mut egui::Ui) {
        let rect = ui.available_rect_before_wrap();
        let line_number_bg = self.theme.editor_background().linear_multiply(0.8);

        ui.painter().rect_filled(rect, egui::Rounding::ZERO, line_number_bg);
        ui.painter().line_segment(
            [egui::pos2(rect.max.x, rect.min.y), egui::pos2(rect.max.x, rect.max.y)],
            egui::Stroke::new(1.0, self.theme.text_color().linear_multiply(0.3))
        );
    }

    fn configure_line_number_spacing(&self, ui: &mut egui::Ui) {
        ui.style_mut().spacing.item_spacing.y = 0.0;
        ui.style_mut().spacing.button_padding.y = 0.0;
    }

    fn draw_line_numbers(&self, ui: &mut egui::Ui, line_count: usize, line_height: f32, line_number_width: f32) {
        for line_num in 1..=line_count {
            ui.allocate_ui_with_layout(
                egui::vec2(line_number_width - 8.0, line_height),
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    ui.label(
                        egui::RichText::new(line_num.to_string())
                            .font(egui::FontId::monospace(self.theme.typography.code_font_size))
                            .color(self.theme.text_color().linear_multiply(0.6))
                    );
                },
            );
        }
    }

    fn calculate_editor_dimensions(&mut self, ui: &egui::Ui, line_count: usize, line_height: f32) -> (f32, f32) {
        let max_line_width = self.get_max_line_width(ui);
        let editor_width = max_line_width.max(ui.available_width());
        let editor_height = line_height * line_count as f32;

        (editor_width, editor_height)
    }

    fn render_text_editor(
        &mut self,
        ui: &mut egui::Ui,
        editor_width: f32,
        editor_height: f32,
    ) {
        let desired_rows = self.code.lines().count().max(1);
        let code_for_highlighting = self.code.clone();

        let layout_job = self.get_highlighted_layout(&code_for_highlighting);
        let response = ui.add_sized(
            [editor_width, editor_height],
            egui::TextEdit::multiline(&mut self.code)
                .font(egui::TextStyle::Monospace)
                .code_editor()
                .desired_rows(desired_rows)
                .desired_width(editor_width)
                .layouter(&mut |ui: &egui::Ui, _buf: &dyn egui::TextBuffer, _wrap_width: f32| {
                    ui.fonts(|f| f.layout_job(layout_job.clone()))
                }),
        );

        if response.changed() {
            self.document_version += 1;
            self.invalidate_caches();
        }

        if !response.has_focus() && response.hovered() {
            response.request_focus();
        }
    }
}