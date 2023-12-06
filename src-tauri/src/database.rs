use rusqlite::{Connection, named_params};
use tauri::AppHandle;
use std::fs;
use crate::accounts::Account;

const CURRENT_DB_VERSION: u32 = 1;

/// Initializes the database connection, creating the .sqlite file if needed, and upgrading the database
/// if it's out of date.
pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle.path_resolver().app_data_dir().expect("The app data directory should exist.");

    println!("App data directory: {:?}", app_dir);

    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let sqlite_path = app_dir.join("BMM.db");

    let mut db = Connection::open(sqlite_path)?;

    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| { Ok(row.get(0)?) })?;
    drop(user_pragma);

    upgrade_database_if_needed(&mut db, existing_user_version)?;

    Ok(db)
}

/// Upgrades the database to the current version.
pub fn upgrade_database_if_needed(db: &mut Connection, existing_version: u32) -> Result<(), rusqlite::Error> {
    if existing_version < CURRENT_DB_VERSION {
        db.pragma_update(None, "journal_mode", "WAL")?;

        let tx = db.transaction()?;

        tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;

        tx.execute_batch(
        "
                CREATE TABLE IF NOT EXISTS accounts
                (
                    id                  INTEGER PRIMARY KEY NOT NULL,
                    name                VARCHAR(40)         NOT NULL,
                    bank_name           VARCHAR(40)         NOT NULL,
                    account_type        INTEGER             NOT NULL,
                    balance             NUMERIC             NOT NULL,
                    available_balance   NUMERIC             NOT NULL,
                    favorite            BOOLEAN             NOT NULL,
                    date_created        DATE                NOT NULL
                );

                CREATE TABLE IF NOT EXISTS transactions
                (
                    id                  INTEGER PRIMARY KEY NOT NULL,
                    payee               VARCHAR(40)         NOT NULL,
                    amount              NUMERIC             NOT NULL,
                    date                DATE                NOT NULL,
                    number              INTEGER,
                    category            VARCHAR(20),
                    memo                VARCHAR(255)
                );
            "
        )?;

        tx.commit()?;
    }

    Ok(())
}

pub fn add_account(account: &Account, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut statement = db.prepare(
        "
            INSERT INTO accounts (name, bank_name, account_type, balance, available_balance,
                favorite, date_created)
            VALUES (@name, @bank_name, @account_type, @balance, @available_balance,
                @favorite, @date_created)
        "
    )?;

    statement.execute(named_params! {
        "@name": account.name,
        "@bank_name": account.bank_name,
        "@account_type": account.account_type,
        "@balance": account.balance,
        "@available_balance": account.available_balance,
        "@favorite": account.favorite,
        "@date_created": account.date_created
    })?;

    Ok(())
}

pub fn get_accounts(db: &Connection) -> Result<Vec<Account>, rusqlite::Error> {
    let mut statement = db.prepare(
        "
            SELECT id, name, bank_name, account_type, balance, available_balance, favorite, date_created
            FROM accounts
        "
    )?;

    let accounts = statement.query_map([], |row| {
        Ok(Account {
            id: row.get(0)?,
            name: row.get(1)?,
            bank_name: row.get(2)?,
            account_type: row.get(3)?,
            balance: row.get(4)?,
            available_balance: row.get(5)?,
            favorite: row.get(6)?,
            date_created: row.get(7)?
        })
    })?;

    let mut accounts_vec = Vec::new();

    for account in accounts {
        accounts_vec.push(account?);
    }

    Ok(accounts_vec)
}

pub fn update_account(account: &Account, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut statement = db.prepare(
        "
            UPDATE accounts
            SET name = @name, bank_name = @bank_name, account_type = @account_type,
                balance = @balance, available_balance = @available_balance, favorite = @favorite,
                date_created = @date_created
            WHERE id = @id
        "
    )?;

    statement.execute(named_params! {
        "@id": account.id,
        "@name": account.name,
        "@bank_name": account.bank_name,
        "@account_type": account.account_type,
        "@balance": account.balance,
        "@available_balance": account.available_balance,
        "@favorite": account.favorite,
        "@date_created": account.date_created
    })?;

    Ok(())
}