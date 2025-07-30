use serde::{Deserialize, Serialize};
use egui::Color32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZenTheme {
    pub name: String,
    pub colors: ThemeColors,
    pub spacing: ThemeSpacing,
    pub typography: ThemeTypography,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub window_bg: [u8; 3],
    pub panel_bg: [u8; 3],
    pub editor_bg: [u8; 3],
    pub faint_bg: [u8; 3],

    pub text_primary: [u8; 3],
    pub text_secondary: [u8; 3],
    pub text_disabled: [u8; 3],

    pub button_bg: [u8; 3],
    pub button_hover: [u8; 3],
    pub button_active: [u8; 3],
    pub selection: [u8; 3],
    pub separator: [u8; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSpacing {
    pub item_spacing: [f32; 2],
    pub button_padding: [f32; 2],
    pub window_margin: f32,
    pub panel_margin: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeTypography {
    pub font_size: f32,
    pub code_font_size: f32,
}

impl Default for ZenTheme {
    fn default() -> Self {
        Self::dark_theme()
    }
}

impl ZenTheme {
    pub fn dark_theme() -> Self {
        Self {
            name: "Dark".to_string(),
            colors: ThemeColors {
                window_bg: [28, 31, 36],
                panel_bg: [32, 35, 42],
                editor_bg: [36, 39, 46],
                faint_bg: [40, 44, 52],
                text_primary: [171, 178, 191],
                text_secondary: [140, 148, 165],
                text_disabled: [100, 108, 125],
                button_bg: [60, 66, 79],
                button_hover: [75, 82, 99],
                button_active: [90, 98, 117],
                selection: [68, 136, 204],
                separator: [60, 66, 79],
            },
            spacing: ThemeSpacing {
                item_spacing: [8.0, 6.0],
                button_padding: [12.0, 6.0],
                window_margin: 8.0,
                panel_margin: 6,
            },
            typography: ThemeTypography {
                font_size: 14.0,
                code_font_size: 13.0,
            },
        }
    }

    pub fn light_theme() -> Self {
        Self {
            name: "Light".to_string(),
            colors: ThemeColors {
                window_bg: [248, 249, 250],
                panel_bg: [255, 255, 255],
                editor_bg: [252, 253, 254],
                faint_bg: [240, 242, 245],
                text_primary: [33, 37, 41],
                text_secondary: [73, 80, 87],
                text_disabled: [134, 142, 150],
                button_bg: [233, 236, 239],
                button_hover: [222, 226, 230],
                button_active: [201, 203, 207],
                selection: [13, 110, 253],
                separator: [222, 226, 230],
            },
            spacing: ThemeSpacing {
                item_spacing: [8.0, 6.0],
                button_padding: [12.0, 6.0],
                window_margin: 8.0,
                panel_margin: 6,
            },
            typography: ThemeTypography {
                font_size: 14.0,
                code_font_size: 13.0,
            },
        }
    }

    pub fn load_available_themes() -> Vec<ZenTheme> {
        let current_dir = std::env::current_dir().unwrap();
        let themes_dir = current_dir.join("themes");

        let mut themes = vec![
            Self::dark_theme(),
            Self::light_theme(),
        ];

        if !themes_dir.exists() {
            return themes;
        }

        let entries = match std::fs::read_dir(themes_dir) {
            Ok(e) => e,
            Err(_) => return themes
        };

        for entry in entries.flatten() {
            let binding = entry.path();
            let ext = match binding.extension() {
                Some(e) => e,
                None => continue
            };

            if ext != "json" {
                continue;
            }

            if let Ok(theme) = Self::load_from_file(&entry.path().to_string_lossy()) {
                themes.push(theme);
            }
        }

        themes
    }

    pub fn apply_to_context(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        style.visuals.window_fill = Color32::from_rgb(
            self.colors.window_bg[0],
            self.colors.window_bg[1],
            self.colors.window_bg[2],
        );
        style.visuals.panel_fill = Color32::from_rgb(
            self.colors.panel_bg[0],
            self.colors.panel_bg[1],
            self.colors.panel_bg[2],
        );
        style.visuals.faint_bg_color = Color32::from_rgb(
            self.colors.faint_bg[0],
            self.colors.faint_bg[1],
            self.colors.faint_bg[2],
        );

        let text_color = self.text_color();

        style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(
            self.colors.button_bg[0],
            self.colors.button_bg[1],
            self.colors.button_bg[2],
        );
        style.visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(
            self.colors.button_bg[0],
            self.colors.button_bg[1],
            self.colors.button_bg[2],
        );
        style.visuals.widgets.inactive.fg_stroke.color = text_color;

        style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(
            self.colors.button_hover[0],
            self.colors.button_hover[1],
            self.colors.button_hover[2],
        );
        style.visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(
            self.colors.button_hover[0],
            self.colors.button_hover[1],
            self.colors.button_hover[2],
        );
        style.visuals.widgets.hovered.fg_stroke.color = text_color;

        style.visuals.widgets.active.bg_fill = Color32::from_rgb(
            self.colors.button_active[0],
            self.colors.button_active[1],
            self.colors.button_active[2],
        );
        style.visuals.widgets.active.weak_bg_fill = Color32::from_rgb(
            self.colors.button_active[0],
            self.colors.button_active[1],
            self.colors.button_active[2],
        );
        style.visuals.widgets.active.fg_stroke.color = text_color;

        style.visuals.widgets.noninteractive.fg_stroke.color = text_color;
        style.visuals.widgets.open.fg_stroke.color = text_color;

        style.visuals.extreme_bg_color = self.editor_background();
        style.visuals.code_bg_color = self.editor_background();
        style.visuals.text_cursor.stroke.color = self.text_color();
        style.visuals.selection.bg_fill = Color32::from_rgb(
            self.colors.selection[0],
            self.colors.selection[1],
            self.colors.selection[2],
        ).linear_multiply(0.3);

        style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::NONE;
        style.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
        style.visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
        style.visuals.widgets.active.bg_stroke = egui::Stroke::NONE;
        style.visuals.window_stroke = egui::Stroke::NONE;

        style.spacing.item_spacing = egui::vec2(
            self.spacing.item_spacing[0],
            self.spacing.item_spacing[1],
        );
        style.spacing.button_padding = egui::vec2(
            self.spacing.button_padding[0],
            self.spacing.button_padding[1],
        );

        ctx.set_style(style);
    }

    pub fn editor_background(&self) -> Color32 {
        Color32::from_rgb(
            self.colors.editor_bg[0],
            self.colors.editor_bg[1],
            self.colors.editor_bg[2],
        )
    }

    pub fn text_color(&self) -> Color32 {
        Color32::from_rgb(
            self.colors.text_primary[0],
            self.colors.text_primary[1],
            self.colors.text_primary[2],
        )
    }

    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let theme: ZenTheme = serde_json::from_str(&json)?;
        Ok(theme)
    }
}