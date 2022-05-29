#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod menu;

fn main() {
  tauri::Builder::default()
    .menu(menu::create_menu())
    .on_menu_event(menu::handle_menu_events())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
