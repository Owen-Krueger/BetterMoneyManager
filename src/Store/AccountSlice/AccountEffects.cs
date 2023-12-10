using BetterMoneyManager.Models;
using Fluxor;
using Microsoft.JSInterop;

namespace BetterMoneyManager.Store.AccountSlice;

public class AccountEffects(IJSRuntime jsRuntime)
{
    [EffectMethod]
    public async Task OnAddAccount(AddAccountAction action, IDispatcher dispatcher)
    {
        await jsRuntime.InvokeVoidAsync("add_account", action.Account);
    }

    [EffectMethod(typeof(GetAccountsRequestedAction))]
    public async Task OnGetAccounts(IDispatcher dispatcher)
    {
        var accounts = await jsRuntime.InvokeAsync<List<Account>>("get_accounts");
        dispatcher.Dispatch(new GetAccountsSucceededAction(accounts));
    }
}