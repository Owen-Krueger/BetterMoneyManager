use tauri::{Menu, MenuItem, Submenu};

pub fn create_menu() -> Menu
{
    let submenu = Submenu::new("File",
        Menu::new().add_native_item(MenuItem::Quit));

    Menu::new()
        .add_submenu(submenu)
}