#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod config;
mod ui;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init();

    if !cfg!(debug_assertions) {
        setup_instance();
    }

    let icon_data = load_icon();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 400.0])
            .with_min_inner_size([500.0, 250.0])
            .with_decorations(false)
            .with_resizable(true)
            .with_transparent(true)
            .with_icon(icon_data),
        ..Default::default()
    };

    eframe::run_native(
        "Zen",
        options,
        Box::new(|_cc| Ok(Box::new(app::ZenEditor::default()))),
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn setup_instance() {
    let path = dirs::home_dir().expect("Failed to locate home directory").join(".zen");
    let themes_dir = path.join("themes");
    let config_json = path.join("config.json");

    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create instance directory");
    }

    if !themes_dir.exists() {
        std::fs::create_dir_all(&themes_dir).expect("Failed to create themes directory");
    }

    if !themes_dir.is_dir() {
        std::fs::remove_file(&themes_dir).expect("Failed to remove invalid config directory");
        std::fs::create_dir_all(&themes_dir).expect("Failed to create themes directory");
    }

    if !config_json.exists() {
        std::fs::write(
            &config_json,
            r#"{"default_theme": "Dark"}"#,
        )
            .expect("Failed to create config file");
    }

    std::env::set_current_dir(&path).expect("Failed to use instance directory");
}

#[cfg(not(target_arch = "wasm32"))]
fn load_icon() -> egui::IconData {
    let icon_bytes = include_bytes!("../assets/zen_logo.png");

    match image::load_from_memory(icon_bytes) {
        Ok(image) => {
            let image = image.to_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();

            egui::IconData {
                rgba,
                width,
                height,
            }
        }
        Err(e) => {
            log::warn!("Failed to load embedded icon: {}", e);
            egui::IconData {
                rgba: vec![255; 32 * 32 * 4],
                width: 32,
                height: 32,
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast;

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let canvas = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("the_canvas_id"))
            .and_then(|e| e.dyn_into::<web_sys::HtmlCanvasElement>().ok());

        if let Some(canvas) = canvas {
            let start_result = eframe::WebRunner::new()
                .start(
                    canvas,
                    web_options,
                    Box::new(|_cc| Ok(Box::new(app::ZenEditor::default()))),
                )
                .await;

            let loading_text = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id("loading"));
            if let Some(loading_text) = loading_text {
                loading_text.remove();
            }

            if let Err(e) = start_result {
                panic!("Failed to start eframe: {e:?}");
            }
        } else {
            panic!("Failed to find canvas element with id 'the_canvas_id'");
        }
    });
}