use std::fs;
use std::path::Path;
use std::sync::Mutex;
use tauri::{Emitter, Manager};

struct AppState {
    pending_file: Mutex<Option<String>>,
}

const MARKDOWN_EXTENSIONS: [&str; 4] = ["md", "markdown", "mdown", "mkdn"];
const MAX_FILE_SIZE_BYTES: u64 = 10 * 1024 * 1024;

fn is_markdown_path(arg: &str) -> bool {
    let lower = arg.to_lowercase();

    MARKDOWN_EXTENSIONS
        .iter()
        .any(|ext| lower.ends_with(&format!(".{ext}")))
}

fn extract_markdown_path(args: &[String]) -> Option<String> {
    args.iter().find(|arg| is_markdown_path(arg)).cloned()
}

#[tauri::command]
fn get_pending_file(state: tauri::State<AppState>) -> Option<String> {
    state.pending_file.lock().unwrap().clone()
}

#[tauri::command]
fn read_markdown_file(path: String) -> Result<String, String> {
    let file = Path::new(&path);

    let is_markdown = file
        .extension()
        .map(|ext| ext.to_string_lossy().to_lowercase())
        .map(|ext| MARKDOWN_EXTENSIONS.contains(&ext.as_str()))
        .unwrap_or(false);

    if !is_markdown {
        return Err(format!("No es un archivo Markdown: {path}"));
    }

    if file.metadata().map_err(|err| err.to_string())?.len() > MAX_FILE_SIZE_BYTES {
        return Err(format!("El archivo supera el límite de 10 MB: {path}"));
    }

    fs::read_to_string(file).map_err(|err| format!("No se pudo leer {path}: {err}"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let initial_args: Vec<String> = std::env::args().collect();

    let app_state = AppState {
        pending_file: Mutex::new(extract_markdown_path(&initial_args)),
    };

    tauri::Builder::default()
        .manage(app_state)
        // Debe registrarse como primer plugin:
        // la segunda instancia termina antes de inicializar los demás.
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            if let Some(path) = extract_markdown_path(&argv) {
                *app.state::<AppState>().pending_file.lock().unwrap() = Some(path.clone());

                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }

                let _ = app.emit("open-file", path);
            }
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_pending_file,
            read_markdown_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
