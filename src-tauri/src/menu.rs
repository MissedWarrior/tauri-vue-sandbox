use tauri::{CustomMenuItem, Menu, Runtime, Submenu, WindowMenuEvent};

pub fn create_menu() -> Menu {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let close = CustomMenuItem::new("close".to_string(), "Close");
  let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
  let menu = Menu::new()
    .add_submenu(submenu);

  menu
}

pub fn handle_menu_events<R: Runtime>() -> impl Fn(WindowMenuEvent<R>) + Send + Sync + 'static {
  |event| {
    match event.menu_item_id() {
      "quit" => {
        println!("test");
      }
      _ => {}
    }
  }
}
