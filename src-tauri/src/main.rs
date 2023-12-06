// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod menus;
mod commands;
mod database;
mod state;
mod accounts;

use state::{AppState};
use tauri::{State, Manager};

fn main() {
  tauri::Builder::default()
      .manage(AppState { db: Default::default() })
      .menu(menus::create_menu())
      .invoke_handler(tauri::generate_handler![
          commands::add_account,
          commands::get_accounts,
          commands::update_account
      ])
      .setup(|app| {
          let handle = app.handle();

          let app_state: State<AppState> = handle.state();
          let db = database::initialize_database(&handle).expect("Database initialize should succeed");
          *app_state.db.lock().unwrap() = Some(db);

          Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}