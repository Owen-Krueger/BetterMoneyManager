using BetterMoneyManager.Models;
using Fluxor;

namespace BetterMoneyManager.Store.AccountSlice;

[FeatureState]
public record AccountState
{
    public bool IsLoading { get; init; }

    public List<Account> Accounts { get; init; } = new();
}