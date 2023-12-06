const app=window.__TAURI__.app;
const dialog=window.__TAURI__.dialog;
const http=window.__TAURI__.http;
const notification=window.__TAURI__.notification;
const invoke = window.__TAURI__.invoke;

/**
 * Invokes `my_custom_command` in the backend.
 * @param {string} message to send to the backend.
 * @returns {string} the response from the backend.
 */
async function my_custom_command(message) {
    return await invoke("my_custom_command", { message });
}

async function get_accounts() {
    return await invoke("get_accounts");
}

async function add_account(account) {
    return await invoke("add_account", { account });
}