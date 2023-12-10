using BetterMoneyManager.Models;
using Fluxor;

namespace BetterMoneyManager.Store.AccountSlice;

/// <summary>
/// The state for the account feature.
/// </summary>
[FeatureState]
public record AccountState
{
    /// <summary>
    /// A flag indicating whether accounts are being loaded.
    /// </summary>
    public bool IsLoading { get; init; }

    /// <summary>
    /// A list of accounts.
    /// </summary>
    public List<Account> Accounts { get; init; } = new();

    public Account SelectedAccount { get; set; } = new();
}