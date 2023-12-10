using BetterMoneyManager.Models;
using Fluxor;

namespace BetterMoneyManager.Store.AccountSlice;

public class AccountReducers
{
    [ReducerMethod(typeof(AddAccountAction))]
    public static AccountState OnAddAccount(AccountState state)
    {
        return state;// with { Count = state.Count + 1 };
    }

    [ReducerMethod(typeof(GetAccountsRequestedAction))]
    public static AccountState OnGetAccountsRequested(AccountState state)
    {
        return state with { IsLoading = true, Accounts = new List<Account>() };
    }

    [ReducerMethod]
    public static AccountState OnGetAccountsSucceeded(AccountState state, GetAccountsSucceededAction action)
    {
        return state with { IsLoading = false, Accounts = action.Accounts.ToList() };
    }
}