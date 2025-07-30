use eframe::egui;
use crate::config::EditorConfig;
#[cfg(target_arch = "wasm32")]
use std::sync::Arc;
#[cfg(target_arch = "wasm32")]
use std::sync::Mutex;

pub trait ZenView {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub struct ZenEditor {
    pub(crate) code_editor: crate::ui::editor::CodeEditor,
    config: EditorConfig,
    show_settings: bool,
    #[cfg(target_arch = "wasm32")]
    pending_file_content: Arc<Mutex<Option<(String, String)>>>,
}

impl Default for ZenEditor {
    fn default() -> Self {
        let config = EditorConfig::load();
        let mut editor = Self {
            code_editor: crate::ui::editor::CodeEditor::default(),
            config,
            show_settings: false,
            #[cfg(target_arch = "wasm32")]
            pending_file_content: Arc::new(Mutex::new(None)),
        };

        if let Some(theme) = editor.code_editor.available_themes.iter()
            .find(|t| t.name == editor.config.default_theme) {
            editor.code_editor.set_theme(theme.clone());
        }

        editor
    }
}

impl ZenEditor {
    fn should_use_custom_frame(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        return false;

        true
    }

    #[cfg(target_arch = "wasm32")]
    fn handle_pending_file_operations(&mut self) {
        if let Ok(mut pending) = self.pending_file_content.try_lock() {
            if let Some((filename, content)) = pending.take() {
                self.code_editor.code = content;
                self.code_editor.selected_file = Some(std::path::PathBuf::from(filename));
            }
        }
    }

    fn custom_window_frame(
        &mut self,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut ZenEditor, &mut egui::Ui),
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
            outer_margin: egui::Margin::same(0),
            ..Default::default()
        };

        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ctx, |ui| {
                let app_rect = ui.max_rect();
                let title_bar_height = 40.0;

                let title_bar_rect = egui::Rect::from_min_size(
                    app_rect.min,
                    egui::vec2(app_rect.width(), title_bar_height),
                );

                let content_rect = egui::Rect::from_min_size(
                    egui::pos2(app_rect.min.x, app_rect.min.y + title_bar_height),
                    egui::vec2(app_rect.width(), app_rect.height() - title_bar_height),
                );

                self.show_title_bar(ui, title_bar_rect);

                let mut content_ui = ui.new_child(egui::UiBuilder::new().max_rect(content_rect).layout(egui::Layout::top_down(egui::Align::LEFT)));
                add_contents(self, &mut content_ui);
            });
    }

    fn show_title_bar(&mut self, ui: &mut egui::Ui, rect: egui::Rect) {
        let interact = ui.interact(rect, egui::Id::new("title_bar"), egui::Sense::click_and_drag());

        if interact.dragged() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }

        if interact.double_clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(
                !ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false)),
            ));
        }

        ui.painter().rect_filled(
            rect,
            egui::CornerRadius::ZERO,
            ui.style().visuals.panel_fill,
        );

        let mut title_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        title_ui.spacing_mut().item_spacing.x = 8.0;
        title_ui.add_space(4.0);

        title_ui.colored_label(
            title_ui.style().visuals.text_color(),
            "⚡ Zen"
        );

        title_ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
        title_ui.style_mut().visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
        title_ui.style_mut().visuals.widgets.active.weak_bg_fill = egui::Color32::TRANSPARENT;
        title_ui.style_mut().visuals.widgets.hovered.weak_bg_fill = egui::Color32::TRANSPARENT;

        title_ui.menu_button(egui::RichText::new("≡").monospace().size(14.0), |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New File").clicked() {
                    self.code_editor.code.clear();
                    self.code_editor.selected_file = None;
                    ui.close();
                }
                if ui.button("Open File...").clicked() {
                    self.open_file_dialog();
                    ui.close();
                }
                if ui.button("Open Project...").clicked() {
                    self.open_project_dialog();
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
                #[cfg(not(target_arch = "wasm32"))]
                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });

            ui.menu_button("Edit", |ui| {
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

            ui.menu_button("Settings", |ui| {
                if ui.button("Preferences...").clicked() {
                    self.show_settings = true;
                    ui.close();
                }
            });
        });

        egui::warn_if_debug_build(&mut title_ui);

        title_ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(8.0);
            let button_size = egui::vec2(22.0, 22.0);

            let close_response = ui.add_sized(
                button_size,
                egui::Button::new(egui::RichText::new("🗙").size(10.0)).frame(false),
            );
            if close_response.clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }

            let max_response = ui.add_sized(
                button_size,
                egui::Button::new(egui::RichText::new("🗖").size(10.0)).frame(false),
            );
            if max_response.clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(
                    !ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false)),
                ));
            }

            let min_response = ui.add_sized(
                button_size,
                egui::Button::new(egui::RichText::new("🗕").size(10.0)).frame(false),
            );
            if min_response.clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Minimized(true));
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

    #[cfg(not(target_arch = "wasm32"))]
    fn save_current_file(&mut self) {
        if let Some(path) = &self.code_editor.selected_file {
            if let Err(e) = std::fs::write(path, &self.code_editor.code) {
                eprintln!("Failed to save file: {}", e);
            }
        } else {
            self.save_file_as();
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn save_current_file(&mut self) {
        self.save_file_as();
    }

    #[cfg(not(target_arch = "wasm32"))]
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

    #[cfg(target_arch = "wasm32")]
    fn save_file_as(&mut self) {
        let code = self.code_editor.code.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let Some(handle) = rfd::AsyncFileDialog::new()
                .add_filter("Text files", &["txt"])
                .add_filter("Rust files", &["rs"])
                .add_filter("All files", &["*"])
                .save_file()
                .await
            {
                if let Err(e) = handle.write(code.as_bytes()).await {
                    eprintln!("Failed to save file: {}", e);
                }
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn open_file_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text files", &["txt"])
            .add_filter("Rust files", &["rs"])
            .add_filter("All files", &["*"])
            .pick_file()
        {
            self.code_editor.load_file(&path);
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn open_file_dialog(&mut self) {
        let pending = self.pending_file_content.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let Some(handle) = rfd::AsyncFileDialog::new()
                .add_filter("Text files", &["txt"])
                .add_filter("Rust files", &["rs"])
                .add_filter("All files", &["*"])
                .pick_file()
                .await
            {
                let data = handle.read().await;
                let content = String::from_utf8_lossy(&data).to_string();
                let filename = handle.file_name();

                if let Ok(mut pending_lock) = pending.lock() {
                    *pending_lock = Some((filename, content));
                }
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn open_project_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.code_editor.open_project(path);
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn open_project_dialog(&mut self) {
        eprintln!("Project opening not supported in web version");
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(target_arch = "wasm32")]
        self.handle_pending_file_operations();

        self.handle_keyboard_shortcuts(ctx);
        self.show_settings_window(ctx);

        if self.should_use_custom_frame() {
            self.custom_window_frame(ctx, |app, ui| {
                app.code_editor.ui(ui);
            });
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                self.code_editor.ui(ui);
            });
        }
    }
}