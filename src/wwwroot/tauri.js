const app=window.__TAURI__.app;
const dialog=window.__TAURI__.dialog;
const http=window.__TAURI__.http;
const notification=window.__TAURI__.notification;
const invoke = window.__TAURI__.invoke;

async function get_accounts() {
    return await invoke("get_accounts");
}

async function add_account(account) {
    return await invoke("add_account", { account });
}