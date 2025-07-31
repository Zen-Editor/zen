use eframe::epaint::Stroke;
use egui::Color32;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Color, Theme};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZenTheme {
    pub name: String,
    pub colors: ThemeColors,
    pub spacing: ThemeSpacing,
    pub typography: ThemeTypography,
    pub syntax: SyntaxColors,
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
pub struct SyntaxColors {
    pub text: [u8; 3],
    pub keyword: [u8; 3],
    pub literal: [u8; 3],
    pub string: [u8; 3],
    pub punctuation: [u8; 3],
    pub preprocessor: [u8; 3],
    pub format_specifier: [u8; 3],
    pub types: [u8; 3],
    pub variables: [u8; 3],
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
                window_bg: [5, 5, 5],
                panel_bg: [10, 10, 10],
                editor_bg: [2, 2, 2],
                faint_bg: [32, 32, 32],
                text_primary: [204, 204, 204],
                text_secondary: [153, 153, 153],
                text_disabled: [102, 102, 102],
                button_bg: [48, 48, 48],
                button_hover: [64, 64, 64],
                button_active: [80, 80, 80],
                selection: [96, 96, 96],
                separator: [48, 48, 48],
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
            syntax: SyntaxColors {
                text: [204, 204, 204],
                keyword: [168, 85, 247],
                literal: [186, 85, 211],
                string: [218, 112, 214],
                punctuation: [212, 212, 212],
                preprocessor: [147, 112, 219],
                format_specifier: [199, 21, 133],
                types: [129, 140, 248],
                variables: [248, 174, 76],
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
            syntax: SyntaxColors {
                text: [33, 37, 41],
                keyword: [147, 51, 234],
                literal: [9, 134, 88],
                string: [163, 21, 21],
                punctuation: [0, 0, 0],
                preprocessor: [128, 128, 128],
                format_specifier: [148, 148, 148],
                types: [37, 99, 235],
                variables: [217, 119, 6],
            },
        }
    }

    pub fn create_syntect_theme(&self) -> Theme {
        let mut theme = Theme::default();

        theme.name = Some(self.name.clone());

        let bg_color = Color {
            r: self.colors.editor_bg[0],
            g: self.colors.editor_bg[1],
            b: self.colors.editor_bg[2],
            a: 255,
        };

        let text_color = Color {
            r: self.syntax.text[0],
            g: self.syntax.text[1],
            b: self.syntax.text[2],
            a: 255,
        };

        theme.settings.background = Some(bg_color);
        theme.settings.foreground = Some(text_color);
        theme.settings.caret = Some(text_color);
        theme.settings.line_highlight = Some(Color {
            r: self.colors.selection[0],
            g: self.colors.selection[1],
            b: self.colors.selection[2],
            a: 50,
        });

        use syntect::highlighting::{ScopeSelectors, StyleModifier};

        let keyword_color = Color {
            r: self.syntax.keyword[0],
            g: self.syntax.keyword[1],
            b: self.syntax.keyword[2],
            a: 255,
        };

        let string_color = Color {
            r: self.syntax.string[0],
            g: self.syntax.string[1],
            b: self.syntax.string[2],
            a: 255,
        };

        let literal_color = Color {
            r: self.syntax.literal[0],
            g: self.syntax.literal[1],
            b: self.syntax.literal[2],
            a: 255,
        };

        let punctuation_color = Color {
            r: self.syntax.punctuation[0],
            g: self.syntax.punctuation[1],
            b: self.syntax.punctuation[2],
            a: 255,
        };

        let preprocessor_color = Color {
            r: self.syntax.preprocessor[0],
            g: self.syntax.preprocessor[1],
            b: self.syntax.preprocessor[2],
            a: 255,
        };

        let function_color = Color {
            r: self.syntax.format_specifier[0],
            g: self.syntax.format_specifier[1],
            b: self.syntax.format_specifier[2],
            a: 255,
        };

        let types_color = Color {
            r: self.syntax.types[0],
            g: self.syntax.types[1],
            b: self.syntax.types[2],
            a: 255,
        };

        let variables_color = Color {
            r: self.syntax.variables[0],
            g: self.syntax.variables[1],
            b: self.syntax.variables[2],
            a: 255,
        };

        theme.scopes = vec![
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("keyword").unwrap(),
                style: StyleModifier {
                    foreground: Some(keyword_color),
                    background: None,
                    font_style: Some(syntect::highlighting::FontStyle::BOLD),
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("keyword.control").unwrap(),
                style: StyleModifier {
                    foreground: Some(keyword_color),
                    background: None,
                    font_style: Some(syntect::highlighting::FontStyle::BOLD),
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("keyword.operator").unwrap(),
                style: StyleModifier {
                    foreground: Some(keyword_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("storage.type").unwrap(),
                style: StyleModifier {
                    foreground: Some(keyword_color),
                    background: None,
                    font_style: Some(syntect::highlighting::FontStyle::BOLD),
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("storage.modifier").unwrap(),
                style: StyleModifier {
                    foreground: Some(keyword_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("keyword.other").unwrap(),
                style: StyleModifier {
                    foreground: Some(keyword_color),
                    background: None,
                    font_style: Some(syntect::highlighting::FontStyle::BOLD),
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("keyword.other.rust").unwrap(),
                style: StyleModifier {
                    foreground: Some(keyword_color),
                    background: None,
                    font_style: Some(syntect::highlighting::FontStyle::BOLD),
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("keyword.other.use").unwrap(),
                style: StyleModifier {
                    foreground: Some(keyword_color),
                    background: None,
                    font_style: Some(syntect::highlighting::FontStyle::BOLD),
                },
            },

            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("string").unwrap(),
                style: StyleModifier {
                    foreground: Some(string_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("string.quoted").unwrap(),
                style: StyleModifier {
                    foreground: Some(string_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("string.quoted.double").unwrap(),
                style: StyleModifier {
                    foreground: Some(string_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("string.quoted.single").unwrap(),
                style: StyleModifier {
                    foreground: Some(string_color),
                    background: None,
                    font_style: None,
                },
            },

            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("constant").unwrap(),
                style: StyleModifier {
                    foreground: Some(literal_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("constant.numeric").unwrap(),
                style: StyleModifier {
                    foreground: Some(literal_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("constant.language").unwrap(),
                style: StyleModifier {
                    foreground: Some(literal_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("constant.character").unwrap(),
                style: StyleModifier {
                    foreground: Some(literal_color),
                    background: None,
                    font_style: None,
                },
            },

            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("entity.name.function").unwrap(),
                style: StyleModifier {
                    foreground: Some(function_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("support.function").unwrap(),
                style: StyleModifier {
                    foreground: Some(function_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("meta.function-call").unwrap(),
                style: StyleModifier {
                    foreground: Some(function_color),
                    background: None,
                    font_style: None,
                },
            },

            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("entity.name.type").unwrap(),
                style: StyleModifier {
                    foreground: Some(types_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("entity.name.class").unwrap(),
                style: StyleModifier {
                    foreground: Some(types_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("entity.name.type.struct").unwrap(),
                style: StyleModifier {
                    foreground: Some(types_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("entity.name.type.enum").unwrap(),
                style: StyleModifier {
                    foreground: Some(types_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("support.type").unwrap(),
                style: StyleModifier {
                    foreground: Some(types_color),
                    background: None,
                    font_style: None,
                },
            },

            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("variable").unwrap(),
                style: StyleModifier {
                    foreground: Some(variables_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("variable.parameter").unwrap(),
                style: StyleModifier {
                    foreground: Some(variables_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("variable.other").unwrap(),
                style: StyleModifier {
                    foreground: Some(variables_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("variable.other.member").unwrap(),
                style: StyleModifier {
                    foreground: Some(variables_color),
                    background: None,
                    font_style: None,
                },
            },

            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("meta.macro").unwrap(),
                style: StyleModifier {
                    foreground: Some(preprocessor_color),
                    background: None,
                    font_style: Some(syntect::highlighting::FontStyle::BOLD),
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("entity.name.function.macro").unwrap(),
                style: StyleModifier {
                    foreground: Some(preprocessor_color),
                    background: None,
                    font_style: Some(syntect::highlighting::FontStyle::BOLD),
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("support.function.macro").unwrap(),
                style: StyleModifier {
                    foreground: Some(preprocessor_color),
                    background: None,
                    font_style: Some(syntect::highlighting::FontStyle::BOLD),
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("meta.attribute").unwrap(),
                style: StyleModifier {
                    foreground: Some(preprocessor_color),
                    background: None,
                    font_style: None,
                },
            },

            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("entity.name.namespace").unwrap(),
                style: StyleModifier {
                    foreground: Some(function_color),
                    background: None,
                    font_style: None,
                },
            },

            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("punctuation").unwrap(),
                style: StyleModifier {
                    foreground: Some(punctuation_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("punctuation.separator").unwrap(),
                style: StyleModifier {
                    foreground: Some(punctuation_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("punctuation.terminator").unwrap(),
                style: StyleModifier {
                    foreground: Some(punctuation_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("punctuation.definition").unwrap(),
                style: StyleModifier {
                    foreground: Some(punctuation_color),
                    background: None,
                    font_style: None,
                },
            },

            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("punctuation.definition.lifetime").unwrap(),
                style: StyleModifier {
                    foreground: Some(keyword_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("entity.name.lifetime").unwrap(),
                style: StyleModifier {
                    foreground: Some(keyword_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("storage.modifier.lifetime").unwrap(),
                style: StyleModifier {
                    foreground: Some(keyword_color),
                    background: None,
                    font_style: None,
                },
            },

            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("meta.preprocessor").unwrap(),
                style: StyleModifier {
                    foreground: Some(preprocessor_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("keyword.other.directive").unwrap(),
                style: StyleModifier {
                    foreground: Some(preprocessor_color),
                    background: None,
                    font_style: None,
                },
            },

            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("support.type.primitive").unwrap(),
                style: StyleModifier {
                    foreground: Some(types_color),
                    background: None,
                    font_style: None,
                },
            },
            syntect::highlighting::ThemeItem {
                scope: ScopeSelectors::from_str("meta.use").unwrap(),
                style: StyleModifier {
                    foreground: Some(text_color),
                    background: None,
                    font_style: None,
                },
            },
        ];

        theme
    }

    pub fn highlight_code(&self, code: &str, language: &str) -> egui::text::LayoutJob {
        let syntax_set = SyntaxSet::load_defaults_newlines();

        let syntax = match language {
            "rs" | "rust" => syntax_set.find_syntax_by_name("Rust"),
            "py" | "python" => syntax_set.find_syntax_by_name("Python"),
            "js" | "javascript" => syntax_set.find_syntax_by_name("JavaScript"),
            "ts" | "typescript" => syntax_set.find_syntax_by_name("TypeScript"),
            "c" | "h" => syntax_set.find_syntax_by_name("C"),
            "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx" => syntax_set.find_syntax_by_name("C++"),
            "java" => syntax_set.find_syntax_by_name("Java"),
            "go" => syntax_set.find_syntax_by_name("Go"),
            "json" => syntax_set.find_syntax_by_name("JSON"),
            "toml" => syntax_set.find_syntax_by_name("TOML"),
            "yaml" | "yml" => syntax_set.find_syntax_by_name("YAML"),
            "xml" => syntax_set.find_syntax_by_name("XML"),
            "html" => syntax_set.find_syntax_by_name("HTML"),
            "css" => syntax_set.find_syntax_by_name("CSS"),
            "md" | "markdown" => syntax_set.find_syntax_by_name("Markdown"),
            "sh" | "bash" | "zsh" => syntax_set.find_syntax_by_name("Bash"),
            _ => syntax_set.find_syntax_by_extension(language)
                .or_else(|| syntax_set.find_syntax_by_name(language))
        }.unwrap_or_else(|| syntax_set.find_syntax_plain_text());

        let theme = self.create_syntect_theme();
        let mut highlighter = HighlightLines::new(syntax, &theme);
        let mut job = egui::text::LayoutJob::default();

        for line in LinesWithEndings::from(code) {
            let ranges = highlighter.highlight_line(line, &syntax_set).unwrap();

            for (style, text) in ranges {
                let color = Color32::from_rgba_unmultiplied(
                    style.foreground.r,
                    style.foreground.g,
                    style.foreground.b,
                    style.foreground.a,
                );

                let font_id = egui::FontId::monospace(self.typography.code_font_size);

                job.append(
                    text,
                    0.0,
                    egui::TextFormat {
                        font_id,
                        color,
                        ..Default::default()
                    },
                );
            }
        }

        job
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

        let no_stroke = Stroke::new(0.0, Color32::TRANSPARENT);

        style.visuals.widgets.active.bg_stroke = no_stroke;
        style.visuals.widgets.inactive.bg_stroke = no_stroke;
        style.visuals.widgets.hovered.bg_stroke = no_stroke;
        style.visuals.widgets.open.bg_stroke = no_stroke;
        style.visuals.widgets.noninteractive.bg_stroke = no_stroke;
        style.visuals.window_stroke = no_stroke;
        style.visuals.popup_shadow = egui::Shadow::NONE;
        style.visuals.widgets.noninteractive.fg_stroke = no_stroke;
        style.visuals.resize_corner_size = 0.0;
        style.visuals.selection.stroke = no_stroke;

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

        style.visuals.widgets.noninteractive.bg_stroke = Stroke::NONE;
        style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;
        style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;
        style.visuals.widgets.active.bg_stroke = Stroke::NONE;
        style.visuals.window_stroke = Stroke::NONE;

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