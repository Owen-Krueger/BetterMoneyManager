use tauri::AppHandle;
use crate::accounts::Account;
use crate::database;
use crate::state::ServiceAccess;

#[tauri::command]
pub fn add_account(app_handle: AppHandle, account: Account) -> Result<(), ()> {
    app_handle.db(|db| database::add_account(&account, db).expect("Add account should succeed"));

    Ok(())
}

#[tauri::command]
pub fn update_account(app_handle: AppHandle, account: Account) -> Result<(), ()> {
    app_handle.db(|db| database::update_account(&account, db).expect("Update account should succeed"));

    Ok(())
}

#[tauri::command]
pub fn get_accounts(app_handle: AppHandle) -> Result<Vec<Account>, ()> {
    let accounts = app_handle.db(|db| database::get_accounts(db).expect("Get accounts should succeed"));

    Ok(accounts)
}