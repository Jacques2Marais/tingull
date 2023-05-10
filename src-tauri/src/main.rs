#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn play_sound() {
  println!("Play a sound!");
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![play_sound])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
