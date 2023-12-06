use tauri::AppHandle;
use crate::accounts::Account;
use crate::database;
use crate::state::ServiceAccess;

/// Adds an account to the accounts table.
///
/// # Arguments
///
/// * `app_handle`: App instance containing the database connection.
/// * `account`: The account to add.
///
/// returns: Result<(), String> Ok if the account was added successfully. Err if unsuccessful.
#[tauri::command]
pub fn add_account(app_handle: AppHandle, account: Account) -> Result<(), String> {
    match app_handle
        .db(|db| database::add_account(&account, db)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
}


/// Gets all accounts from the accounts table.
///
/// # Arguments
///
/// * `app_handle`: App instance containing the database connection.
///
/// returns: Result<Vec<Account, Global>, String> List of accounts. Err if unsuccessful.
#[tauri::command]
pub fn get_accounts(app_handle: AppHandle) -> Result<Vec<Account>, String> {
    match app_handle
        .db(|db| database::get_accounts(db)) {
            Ok(accounts) => Ok(accounts),
            Err(e) => Err(e.to_string())
        }
}

/// Updates an account in the accounts table.
///
/// # Arguments
///
/// * `app_handle`: App instance containing the database connection.
/// * `account`: The account to update and its new values.
///
/// returns: Result<(), String> Ok if successful. Err if unsuccessful.
#[tauri::command]
pub fn update_account(app_handle: AppHandle, account: Account) -> Result<(), String> {
    match app_handle
        .db(|db| database::update_account(&account, db)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
}


/// Removes an account from the accounts table.
///
/// # Arguments
///
/// * `app_handle`: App instance containing the database connection.
/// * `account_id`: Id of the account to remove.
///
/// returns: Result<(), ()> Ok if successful. Err if unsuccessful.
#[tauri::command]
pub fn remove_account(app_handle: AppHandle, account_id: i32) -> Result<(), String> {
    match app_handle
        .db(|db| database::remove_account(account_id, db)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
}

/// Adds a transaction to the transactions table.
///
/// # Arguments
///
/// * `app_handle`: App instance containing the database connection.
/// * `transaction`: Transaction to add.
///
/// returns: Result<(), ()> Ok if successful. Err if unsuccessful.
#[tauri::command]
pub fn add_transaction(app_handle: AppHandle, transaction: crate::transactions::Transaction) -> Result<(), String> {
    match app_handle
        .db(|db| database::add_transaction(&transaction, db)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
}

/// Gets all transactions from the transactions table for a given account.
///
/// # Arguments
///
/// * `app_handle`: App instance containing the database connection.
/// * `account_id`: Id of the account to get transactions for.
///
/// returns: Result<Vec<Transaction, Global>, ()> List of transactions. Err if unsuccessful.
#[tauri::command]
pub fn get_transactions(app_handle: AppHandle, account_id: i32) -> Result<Vec<crate::transactions::Transaction>, String> {
    match app_handle
        .db(|db| database::get_transactions(account_id, db)) {
            Ok(transactions) => Ok(transactions),
            Err(e) => Err(e.to_string())
        }
}

/// Updates a transaction in the transactions table.
///
/// # Arguments
///
/// * `app_handle`: App instance containing the database connection.
/// * `transaction`: Transaction to update and its new values.
///
/// returns: Result<(), ()> Ok if successful. Err if unsuccessful.
#[tauri::command]
pub fn update_transaction(app_handle: AppHandle, transaction: crate::transactions::Transaction) -> Result<(), String> {
    match app_handle
        .db(|db| database::update_transaction(&transaction, db)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
}

/// Removes a transaction from the transactions table.
///
/// # Arguments
///
/// * `app_handle`: App instance containing the database connection.
/// * `transaction_id`: Id of the transaction to remove.
///
/// returns: Result<(), ()> Ok if successful. Err if unsuccessful.
#[tauri::command]
pub fn remove_transaction(app_handle: AppHandle, transaction_id: i32) -> Result<(), String> {
    match app_handle
        .db(|db| database::remove_transaction(transaction_id, db)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
}