use state::{AppState};
use tauri::{State, Manager};
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};

mod commands;
mod database;
mod state;
mod accounts;
mod transactions;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    const DEVTOOLS_ID: &str = "devtools";

    tauri::Builder::default()
        .manage(AppState { db: Default::default() })
        .invoke_handler(tauri::generate_handler![
          commands::add_account,
          commands::get_accounts,
          commands::update_account,
          commands::remove_account,
          commands::add_transaction,
          commands::get_transactions,
          commands::update_transaction,
          commands::remove_transaction
      ])
        .setup(|app| {
            let file_menu = SubmenuBuilder::new(app, "File")
                .about(None)
                .quit()
                .build()?;
            let edit_menu = SubmenuBuilder::new(app, "Edit")
                .cut()
                .copy()
                .paste()
                .separator()
                .undo()
                .redo()
                .build()?;
            let tool_menu = SubmenuBuilder::new(app, "Tools")
                .item(&MenuItemBuilder::with_id(DEVTOOLS_ID, "Open Devtools").build(app)?)
                .build()?;
            let menu = MenuBuilder::new(app)
                .item(&file_menu)
                .item(&edit_menu)
                .item(&tool_menu)
                .build()?;
            app.set_menu(menu)?;

            app.on_menu_event(move |app, event| {
                if event.id() == DEVTOOLS_ID {
                    app.get_webview_window("main")
                        .expect("Main window expected")
                        .open_devtools();
                }
            });

            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db = database::initialize_database(&handle)
                .expect("Database initialize should succeed");
            *app_state.db.lock().unwrap() = Some(db);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}