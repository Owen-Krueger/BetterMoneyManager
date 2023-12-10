using BetterMoneyManager.Models;
using Fluxor;
using Microsoft.JSInterop;

namespace BetterMoneyManager.Store.AccountSlice;

/// <summary>
/// Effects to run when account actions are dispatched.
/// </summary>
public class AccountEffects(IJSRuntime jsRuntime)
{
    /// <summary>
    /// Adds a new account to the database.
    /// </summary>
    /// <param name="action">Action containing the account to add.</param>
    [EffectMethod]
    public async Task OnAddAccount(AddAccountAction action)
    {
        await jsRuntime.InvokeVoidAsync("add_account", action.Account);
    }

    /// <summary>
    /// Gets accounts from the database.
    /// </summary>
    /// <param name="dispatcher">Dispatch event when accounts retrieved.</param>
    [EffectMethod(typeof(GetAccountsRequestedAction))]
    public async Task OnGetAccounts(IDispatcher dispatcher)
    {
        var accounts = await jsRuntime.InvokeAsync<List<Account>>("get_accounts");
        dispatcher.Dispatch(new GetAccountsSucceededAction(accounts));
    }
}