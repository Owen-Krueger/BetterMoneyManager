const app=window.__TAURI__.app;
const dialog=window.__TAURI__.dialog;
const http=window.__TAURI__.http;
const notification=window.__TAURI__.notification;
const invoke = window.__TAURI__.invoke;

async function add_account(account) {
    return await invoke("add_account", { account });
}

async function get_accounts() {
    return await invoke("get_accounts");
}

async function update_account(account) {
    return await invoke("update_account", { account });
}

async function remove_account(account_id) {
    return await invoke("remove_account", { account_id });
}