const app=window.__TAURI__.app;
const dialog=window.__TAURI__.dialog;
const http=window.__TAURI__.http;
const notification=window.__TAURI__.notification;
const invoke = window.__TAURI__.invoke;

/**
 * Adds a new account to the database.
 * @param {Object} account - The account to add to the database.
 * @returns {Promise} - A promise that resolves to the account added.
 */
async function add_account(account) {
    return await invoke("add_account", { account });
}

/**
 * Gets all accounts from the database.
 * @returns {Promise<Object>} - A promise that resolves to an array of accounts from the database.
 */
async function get_accounts() {
    return await invoke("get_accounts");
}

/**
 * Updates an account in the database.
 * @param {Object} account - The account to update in the database with updated properties to set.
 * @returns {Promise} - A promise that resolves to the account updated.
 */
async function update_account(account) {
    return await invoke("update_account", { account });
}

/**
 * Removes an account from the database.
 * @param {number} account_id - The id of the account to remove from the database.
 * @returns {Promise} - A promise that resolves to the account removed.
 */
async function remove_account(account_id) {
    return await invoke("remove_account", { account_id });
}

/**
 * Adds a new transaction to the database.
 * @param {Object} transaction - The transaction to add to the database.
 * @returns {Promise} - A promise that resolves to the transaction added.
 */
async function add_transaction(transaction) {
    return await invoke("add_transaction", { transaction });
}

/**
 * Gets all transactions from the database.
 * @returns {Promise<Object>} - A promise that resolves to an array of transactions from the database.
 */
async function get_transactions() {
    return await invoke("get_transactions");
}

/**
 * Updates a transaction in the database.
 * @param {Object} transaction - The transaction to update in the database with updated properties to set.
 * @returns {Promise} - A promise that resolves to the transaction updated.
 */
async function update_transaction(transaction) {
    return await invoke("update_transaction", { transaction });
}

/**
 * Removes a transaction from the database.
 * @param {Number} transaction_id - The id of the transaction to remove from the database.
 * @returns {Promise} - A promise that resolves to the transaction removed.
 */
async function remove_transaction(transaction_id) {
    return await invoke("remove_transaction", { transaction_id });
}