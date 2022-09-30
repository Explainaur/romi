mod actions;
mod ui;

use crate::actions::{decompress, unzip};
use crate::actions::decompress::{unrar::archive};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, Manager, WindowBuilder};
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

fn main() {
  let menu = Menu::new()
    .add_item(CustomMenuItem::new("hide", "Hide"))
    .add_submenu(Submenu::new("View", Menu::new()
      .add_native_item(MenuItem::Zoom)
      .add_native_item(MenuItem::EnterFullScreen)
      .add_native_item(MenuItem::Hide),
    ));

  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_window("main").unwrap();

      #[cfg(target_os = "macos")]
      apply_vibrancy(&window, NSVisualEffectMaterial::Sidebar, None, None)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

      Ok(())
    })
    .menu(menu)
    .invoke_handler(tauri::generate_handler![actions::parse])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}