use rusqlite::{Connection, named_params};
use tauri::AppHandle;
use std::fs;
use crate::accounts::Account;
use crate::transactions::Transaction;

const CURRENT_DB_VERSION: u32 = 1;

/// Sets up the database connection and returns it.
///
/// # Arguments
///
/// * `app_handle`: App instance.
///
/// returns: Result<Connection, Error> The database connection.
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

/// Upgrades the database and sets up tables if needed.
///
/// # Arguments
///
/// * `db`: Database connection.
/// * `existing_version`: Version of the database.
///
/// returns: Result<(), Error> Ok if successful.
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
                    account_id          INTEGER             NOT NULL,
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

/// Adds an account to the accounts table.
///
/// # Arguments
///
/// * `account`: Account to add.
/// * `db`: Database connection.
///
/// returns: Result<(), Error> Ok if successful.
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


/// Gets all accounts from the accounts table.
///
/// # Arguments
///
/// * `db`: Database connection.
///
/// returns: Result<Vec<Account, Global>, Error> List of accounts.
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


/// Updates an account in the accounts table.
///
/// # Arguments
///
/// * `account`: Account to update and its new values.
/// * `db`: Database connection.
///
/// returns: Result<(), Error> Ok if successful.
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

/// Removes an account from the accounts table.
///
/// # Arguments
///
/// * `account_id`: Id of the account to remove.
/// * `db`: Database connection.
///
/// returns: Result<(), Error> Ok if successful.
pub fn remove_account(account_id: i32, db: &Connection) -> Result<(), rusqlite::Error> {

    let mut statement = db.prepare(
        "
            DELETE FROM accounts
            WHERE id = @id
        "
    )?;

    statement.execute(named_params! {
        "@id": account_id
    })?;

    statement = db.prepare(
        "
            DELETE FROM transactions
            WHERE account_id = @account_id
        "
    )?;

    statement.execute(named_params! {
        "@account_id": account_id
    })?;

    Ok(())
}

/// Adds a transaction to the transactions table.
///
/// # Arguments
///
/// * `transaction`: Transaction to add.
/// * `db`: Database connection.
///
/// returns: Result<(), Error> Ok if successful.
pub fn add_transaction(transaction: &Transaction, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut statement = db.prepare(
        "
            INSERT INTO transactions (account_id, payee, amount, date, number, category, memo)
            VALUES (@account_id, @payee, @amount, @date, @number, @category, @memo)
        "
    ).unwrap();

    statement.execute(named_params! {
        "@account_id": transaction.account_id,
        "@payee": transaction.payee,
        "@amount": transaction.amount,
        "@date": transaction.date,
        "@number": transaction.number,
        "@category": transaction.category,
        "@memo": transaction.memo
    }).unwrap();

    Ok(())
}


/// Gets all transactions from the transactions table for a given account.
///
/// # Arguments
///
/// * `account_id`: Id of the account to get transactions for.
/// * `db`: Database connection.
///
/// returns: Result<Vec<Transaction, Global>, Error> List of transactions.
pub fn get_transactions(account_id: i32, db: &Connection) -> Result<Vec<Transaction>, rusqlite::Error> {
    let mut statement = db.prepare(
        "
            SELECT id, account_id, payee, amount, date, number, category, memo
            FROM transactions
            WHERE account_id = @account_id
        "
    ).unwrap();

    let transactions = statement.query_map(named_params! {
        "@account_id": account_id
    }, |row| {
        Ok(Transaction {
            id: row.get(0).unwrap(),
            account_id: row.get(1).unwrap(),
            payee: row.get(2).unwrap(),
            amount: row.get(3).unwrap(),
            date: row.get(4).unwrap(),
            number: row.get(5).unwrap(),
            category: row.get(6).unwrap(),
            memo: row.get(7).unwrap()
        })
    }).unwrap();

    let mut transactions_vec = Vec::new();

    for transaction in transactions {
        transactions_vec.push(transaction.unwrap());
    }

    Ok(transactions_vec)
}

/// Updates a transaction in the transactions table.
///
/// # Arguments
///
/// * `transaction`: Transaction to update and its new values.
/// * `db`: Database connection.
///
/// returns: Result<(), Error> Ok if successful.
pub fn update_transaction(transaction: &Transaction, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut statement = db.prepare(
        "
            UPDATE transactions
            SET account_id = @account_id, payee = @payee, amount = @amount, date = @date,
                number = @number, category = @category, memo = @memo
            WHERE id = @id
        "
    )?;

    statement.execute(named_params! {
        "@id": transaction.id,
        "@account_id": transaction.account_id,
        "@payee": transaction.payee,
        "@amount": transaction.amount,
        "@date": transaction.date,
        "@number": transaction.number,
        "@category": transaction.category,
        "@memo": transaction.memo
    })?;

    Ok(())
}


/// Removes a transaction from the transactions table.
///
/// # Arguments
///
/// * `id`: Id of the transaction to remove.
/// * `db`: Database connection.
///
/// returns: Result<(), Error> Ok if successful.
pub fn remove_transaction(id: i32, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut statement = db.prepare(
        "
            DELETE FROM transactions
            WHERE id = @id
        "
    )?;

    statement.execute(named_params! {
        "@id": id
    })?;

    Ok(())
}