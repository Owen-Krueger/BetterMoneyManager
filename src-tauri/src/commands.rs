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
/// returns: Result<(), ()> Ok if the account was added successfully.
#[tauri::command]
pub fn add_account(app_handle: AppHandle, account: Account) -> Result<(), ()> {
    app_handle
        .db(|db| database::add_account(&account, db)
        .expect("Add account should succeed"));

    Ok(())
}


/// Gets all accounts from the accounts table.
///
/// # Arguments
///
/// * `app_handle`: App instance containing the database connection.
///
/// returns: Result<Vec<Account, Global>, ()> List of accounts.
#[tauri::command]
pub fn get_accounts(app_handle: AppHandle) -> Result<Vec<Account>, ()> {
    let accounts = app_handle
        .db(|db| database::get_accounts(db)
        .expect("Get accounts should succeed"));

    Ok(accounts)
}

/// Updates an account in the accounts table.
///
/// # Arguments
///
/// * `app_handle`: App instance containing the database connection.
/// * `account`: The account to update and its new values.
///
/// returns: Result<(), ()> Ok if successful.
#[tauri::command]
pub fn update_account(app_handle: AppHandle, account: Account) -> Result<(), ()> {
    app_handle
        .db(|db| database::update_account(&account, db)
        .expect("Update account should succeed"));

    Ok(())
}


/// Removes an account from the accounts table.
///
/// # Arguments
///
/// * `app_handle`: App instance containing the database connection.
/// * `account_id`: Id of the account to remove.
///
/// returns: Result<(), ()> Ok if successful.
#[tauri::command]
pub fn remove_account(app_handle: AppHandle, account_id: i32) -> Result<(), ()> {
    app_handle
        .db(|db| database::remove_account(account_id, db)
        .expect("Remove account should succeed"));

    Ok(())
}

/// Adds a transaction to the transactions table.
///
/// # Arguments
///
/// * `app_handle`: App instance containing the database connection.
/// * `transaction`: Transaction to add.
///
/// returns: Result<(), ()> Ok if successful.
#[tauri::command]
pub fn add_transaction(app_handle: AppHandle, transaction: crate::transactions::Transaction) -> Result<(), ()> {
    app_handle
        .db(|db| database::add_transaction(&transaction, db)
        .expect("Add transaction should succeed"));

    Ok(())
}

/// Gets all transactions from the transactions table for a given account.
///
/// # Arguments
///
/// * `app_handle`: App instance containing the database connection.
/// * `account_id`: Id of the account to get transactions for.
///
/// returns: Result<Vec<Transaction, Global>, ()> List of transactions.
#[tauri::command]
pub fn get_transactions(app_handle: AppHandle, account_id: i32) -> Result<Vec<crate::transactions::Transaction>, ()> {
    let transactions = app_handle
        .db(|db| database::get_transactions(account_id, db)
        .expect("Get transactions should succeed"));

    Ok(transactions)
}

/// Updates a transaction in the transactions table.
///
/// # Arguments
///
/// * `app_handle`: App instance containing the database connection.
/// * `transaction`: Transaction to update and its new values.
///
/// returns: Result<(), ()> Ok if successful.
#[tauri::command]
pub fn update_transaction(app_handle: AppHandle, transaction: crate::transactions::Transaction) -> Result<(), ()> {
    app_handle
        .db(|db| database::update_transaction(&transaction, db)
        .expect("Update transaction should succeed"));

    Ok(())
}

/// Removes a transaction from the transactions table.
///
/// # Arguments
///
/// * `app_handle`: App instance containing the database connection.
/// * `transaction_id`: Id of the transaction to remove.
///
/// returns: Result<(), ()> Ok if successful.
#[tauri::command]
pub fn remove_transaction(app_handle: AppHandle, transaction_id: i32) -> Result<(), ()> {
    app_handle
        .db(|db| database::remove_transaction(transaction_id, db)
        .expect("Remove transaction should succeed"));

    Ok(())
}