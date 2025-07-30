use eframe::egui;
use crate::config::EditorConfig;

pub trait ZenView {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub struct ZenEditor {
    pub(crate) code_editor: crate::ui::editor::CodeEditor,
    config: EditorConfig,
    show_settings: bool,
}

impl Default for ZenEditor {
    fn default() -> Self {
        let config = EditorConfig::load();
        let mut editor = Self {
            code_editor: crate::ui::editor::CodeEditor::default(),
            config,
            show_settings: false,
        };

        if let Some(theme) = editor.code_editor.available_themes.iter()
            .find(|t| t.name == editor.config.default_theme) {
            editor.code_editor.set_theme(theme.clone());
        }

        editor
    }
}

impl ZenEditor {
    fn custom_window_frame(
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
        add_contents: impl FnOnce(&mut egui::Ui)
    ) {
        let panel_frame = egui::Frame {
            fill: ctx.style().visuals.window_fill,
            corner_radius: egui::CornerRadius::same(12),
            shadow: egui::Shadow {
                color: egui::Color32::from_black_alpha(80),
                offset: [0, 8],
                blur: 16,
                spread: 0,
            },
            outer_margin: egui::Margin::same(8),
            ..Default::default()
        };

        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ctx, |ui| {
                let app_rect = ui.max_rect();

                let title_bar_height = 40.0;
                let menu_bar_height = 35.0;

                let title_bar_rect = egui::Rect::from_min_size(
                    app_rect.min,
                    egui::vec2(app_rect.width(), title_bar_height),
                );

                let menu_bar_rect = egui::Rect::from_min_size(
                    egui::pos2(app_rect.min.x, app_rect.min.y + title_bar_height),
                    egui::vec2(app_rect.width(), menu_bar_height),
                );

                let content_rect = egui::Rect::from_min_size(
                    egui::pos2(app_rect.min.x, app_rect.min.y + title_bar_height + menu_bar_height),
                    egui::vec2(app_rect.width(), app_rect.height() - title_bar_height - menu_bar_height),
                );

                Self::show_title_bar(ui, title_bar_rect, frame);

                #[allow(deprecated)]
                ui.allocate_ui_at_rect(menu_bar_rect, |ui| {
                    ui.painter().rect_filled(
                        menu_bar_rect,
                        egui::CornerRadius::ZERO,
                        ui.style().visuals.faint_bg_color,
                    );

                    ui.painter().line_segment(
                        [
                            egui::pos2(menu_bar_rect.min.x, menu_bar_rect.max.y),
                            egui::pos2(menu_bar_rect.max.x, menu_bar_rect.max.y),
                        ],
                        egui::Stroke::new(1.0, ui.style().visuals.widgets.noninteractive.bg_stroke.color),
                    );
                });

                let mut content_ui = ui.new_child(egui::UiBuilder::new().max_rect(content_rect).layout(egui::Layout::top_down(egui::Align::LEFT)));
                add_contents(&mut content_ui);
            });
    }

    fn show_title_bar(ui: &mut egui::Ui, rect: egui::Rect, _frame: &mut eframe::Frame) {
        let interact = ui.interact(rect, egui::Id::new("title_bar"), egui::Sense::click_and_drag());

        if interact.dragged() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }

        if interact.double_clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(!ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false))));
        }

        ui.painter().rect_filled(
            rect,
            egui::CornerRadius::ZERO,
            ui.style().visuals.panel_fill,
        );

        let mut title_ui = ui.new_child(egui::UiBuilder::new().max_rect(rect).layout(egui::Layout::left_to_right(egui::Align::Center)));
        title_ui.spacing_mut().item_spacing.x = 8.0;
        title_ui.add_space(12.0);

        title_ui.colored_label(
            title_ui.style().visuals.text_color(),
            "⚡ Zen Editor"
        );

        title_ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(8.0);

            let button_size = egui::vec2(28.0, 20.0);

            if ui.add_sized(button_size, egui::Button::new("X"))
                .on_hover_text("Close")
                .clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }

            if ui.add_sized(button_size, egui::Button::new("[]"))
                .on_hover_text("Maximize/Restore")
                .clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(!ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false))));
            }

            if ui.add_sized(button_size, egui::Button::new("−"))
                .on_hover_text("Minimize")
                .clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Minimized(true));
            }
        });

        ui.painter().line_segment(
            [
                egui::pos2(rect.min.x, rect.max.y),
                egui::pos2(rect.max.x, rect.max.y),
            ],
            egui::Stroke::new(1.0, ui.style().visuals.widgets.noninteractive.bg_stroke.color),
        );
    }

    fn show_menu_bar(&mut self, ui: &mut egui::Ui, rect: egui::Rect) {
        let mut menu_ui = ui.new_child(egui::UiBuilder::new().max_rect(rect).layout(egui::Layout::left_to_right(egui::Align::Center)));
        menu_ui.spacing_mut().item_spacing.x = 4.0;
        menu_ui.add_space(8.0);

        let style = menu_ui.style_mut();
        style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
        style.visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;

        menu_ui.menu_button("File", |ui| {
            if ui.button("New File").clicked() {
                self.code_editor.code.clear();
                self.code_editor.selected_file = None;
                ui.close();
            }
            if ui.button("Open File...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Text files", &["txt"])
                    .add_filter("Rust files", &["rs"])
                    .add_filter("All files", &["*"])
                    .pick_file() {
                    self.code_editor.load_file(&path);
                }
                ui.close();
            }
            if ui.button("Open Project...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.code_editor.open_project(path);
                }
                ui.close();
            }
            ui.separator();
            if ui.button("Save").clicked() {
                self.save_current_file();
                ui.close();
            }
            if ui.button("Save As...").clicked() {
                self.save_file_as();
                ui.close();
            }
            ui.separator();
            if ui.button("Exit").clicked() {
                std::process::exit(0);
            }
        });

        menu_ui.menu_button("Edit", |ui| {
            ui.add_enabled_ui(false, |ui| {
                let _ = ui.button("Undo");
            });
            ui.add_enabled_ui(false, |ui| {
                let _ = ui.button("Redo");
            });
            ui.separator();
            ui.add_enabled_ui(false, |ui| {
                let _ = ui.button("Cut");
            });
            ui.add_enabled_ui(false, |ui| {
                let _ = ui.button("Copy");
            });
            ui.add_enabled_ui(false, |ui| {
                let _ = ui.button("Paste");
            });
        });

        menu_ui.menu_button("View", |ui| {
            if ui.button("Toggle File Explorer").clicked() {
                ui.close();
            }
        });

        menu_ui.menu_button("Settings", |ui| {
            if ui.button("Preferences...").clicked() {
                self.show_settings = true;
                ui.close();
            }
        });
    }

    fn handle_keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        let is_mac = cfg!(target_os = "macos");
        let control_key = if is_mac { egui::Modifiers::MAC_CMD } else { egui::Modifiers::CTRL };

        if ctx.input(|i| i.modifiers.matches_exact(control_key) && i.key_pressed(egui::Key::S)) {
            self.save_current_file();
        }
    }

    fn save_current_file(&mut self) {
        if let Some(path) = &self.code_editor.selected_file {
            if let Err(e) = std::fs::write(path, &self.code_editor.code) {
                eprintln!("Failed to save file: {}", e);
            }
        } else {
            self.save_file_as();
        }
    }

    fn save_file_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text files", &["txt"])
            .add_filter("Rust files", &["rs"])
            .add_filter("All files", &["*"])
            .save_file() {
            if let Err(e) = std::fs::write(&path, &self.code_editor.code) {
                eprintln!("Failed to save file: {}", e);
            } else {
                self.code_editor.selected_file = Some(path);
            }
        }
    }

    fn show_settings_window(&mut self, ctx: &egui::Context) {
        if !self.show_settings {
            return;
        }

        let mut show = self.show_settings;
        egui::Window::new("Settings")
            .open(&mut show)
            .resizable(true)
            .default_width(400.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Theme Settings");

                    let theme_names: Vec<&str> = self.code_editor.available_themes
                        .iter()
                        .map(|t| t.name.as_str())
                        .collect();

                    let mut selected_idx = self.code_editor.selected_theme_index;
                    ui.horizontal(|ui| {
                        ui.label("Theme:");
                        egui::ComboBox::from_id_salt("settings_theme_selector")
                            .selected_text(theme_names[selected_idx])
                            .show_ui(ui, |ui| {
                                for (idx, name) in theme_names.iter().enumerate() {
                                    ui.selectable_value(&mut selected_idx, idx, *name);
                                }
                            });
                    });

                    if selected_idx != self.code_editor.selected_theme_index {
                        self.code_editor.selected_theme_index = selected_idx;
                        if let Some(theme) = self.code_editor.available_themes.get(selected_idx) {
                            self.code_editor.set_theme(theme.clone());
                        }
                    }

                    ui.checkbox(&mut false, "Dark mode follows system");

                    if ui.button("Set as Default Theme").clicked() {
                        self.config.default_theme = self.code_editor.theme.name.clone();
                        if let Err(e) = self.config.save() {
                            eprintln!("Failed to save config: {}", e);
                        }
                    }

                    ui.separator();

                    ui.heading("Editor Settings");

                    let mut font_size = self.code_editor.theme.typography.code_font_size;
                    if ui.add(egui::Slider::new(&mut font_size, 10.0..=24.0).text("Font Size")).changed() {
                        let mut new_theme = self.code_editor.theme.clone();
                        new_theme.typography.code_font_size = font_size;
                        new_theme.typography.font_size = font_size + 1.0;
                        self.code_editor.set_theme(new_theme);
                    }
                });
            });

        self.show_settings = show;
    }
}

impl eframe::App for ZenEditor {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.handle_keyboard_shortcuts(ctx);
        self.show_settings_window(ctx);

        let menu_bar_rect = {
            let app_rect = ctx.available_rect();
            let title_bar_height = 40.0;
            let menu_bar_height = 35.0;
            egui::Rect::from_min_size(
                egui::pos2(app_rect.min.x + 8.0, app_rect.min.y + title_bar_height + 8.0),
                egui::vec2(app_rect.width() - 16.0, menu_bar_height),
            )
        };

        egui::Area::new(egui::Id::new("menu_area"))
            .fixed_pos(menu_bar_rect.min)
            .show(ctx, |ui| {
                self.show_menu_bar(ui, menu_bar_rect);
            });

        Self::custom_window_frame(ctx, frame, |ui| {
            self.code_editor.ui(ui);
        });
    }
}