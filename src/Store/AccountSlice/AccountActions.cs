using BetterMoneyManager.Models;

namespace BetterMoneyManager.Store.AccountSlice;

/// <summary>
/// Adds a new account to the database.
/// </summary>
/// <param name="Account">Account to add.</param>
public record AddAccountAction(Account Account);

/// <summary>
/// Requests that accounts are retrieved from the database.
/// </summary>
public record GetAccountsRequestedAction;

/// <summary>
/// Sets accounts in the state after they have been retrieved from the database.
/// </summary>
/// <param name="Accounts">Accounts from the database.</param>
public record GetAccountsSucceededAction(IEnumerable<Account> Accounts);

public record EditAccountRequestedAction(Account Account);

public record UpdateAccountAction(Account Account);