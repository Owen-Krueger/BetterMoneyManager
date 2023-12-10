using BetterMoneyManager.Models;

namespace BetterMoneyManager.Store.AccountSlice;

public record AddAccountAction(Account Account);

public record GetAccountsRequestedAction;

public record GetAccountsSucceededAction(IEnumerable<Account> Accounts);