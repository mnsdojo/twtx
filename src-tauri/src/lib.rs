use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};
use tauri::{AppHandle, Emitter};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn download_video(
    app: AppHandle,
    url: String,
    quality: String,
    output_path: String,
) -> Result<String, String> {
    let mut cmd = Command::new("yt-dlp")
        .arg("-f")
        .arg(&quality)
        .arg("-P")
        .arg(&output_path)
        .arg("--merge-output-format")
        .arg("mp4")
        .arg(&url)
        .stdout(Stdio::null()) // progress comes on stderr
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start yt-dlp: {}", e))?;

    if let Some(stderr) = cmd.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines().flatten() {
            if let Some(progress) = parse_progress(&line) {
                // Emit progress to frontend
                let _ = app.emit("download_progress", progress);
            }
        }
    }

    let status = cmd.wait().map_err(|e| format!("yt-dlp failed: {}", e))?;

    if status.success() {
        Ok("✅ Download completed successfully!".into())
    } else {
        Err("❌ yt-dlp failed during download.".into())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, download_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn parse_progress(line: &str) -> Option<String> {
    if line.contains("[download]") && line.contains('%') {
        let parts: Vec<&str> = line.split_whitespace().collect();
        for part in parts {
            if part.ends_with('%') {
                return Some(part.trim().to_string()); // e.g., "25.3%"
            }
        }
    }
    None
}
