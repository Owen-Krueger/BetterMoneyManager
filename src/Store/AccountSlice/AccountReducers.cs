using BetterMoneyManager.Models;
using Fluxor;

namespace BetterMoneyManager.Store.AccountSlice;

/// <summary>
/// Reducer methods to handle account actions.
/// </summary>
public class AccountReducers
{
    /// <summary>
    /// Request to add a new account.
    /// </summary>
    /// <param name="state">Account state.</param>
    /// <returns>Updated account state.</returns>
    [ReducerMethod(typeof(GetAccountsRequestedAction))]
    public static AccountState OnGetAccountsRequested(AccountState state)
    {
        return state with { IsLoading = true, Accounts = new List<Account>() };
    }

    /// <summary>
    /// Sets accounts in the state after they have been retrieved from the database.
    /// </summary>
    /// <param name="state">Account state.</param>
    /// <param name="action">Action containing accounts from the database.</param>
    /// <returns>Updated account state.</returns>
    [ReducerMethod]
    public static AccountState OnGetAccountsSucceeded(AccountState state, GetAccountsSucceededAction action)
    {
        return state with { IsLoading = false, Accounts = action.Accounts.ToList() };
    }
}