using BetterMoneyManager.Models;
using Fluxor;

namespace BetterMoneyManager.Store.AppSlice;

/// <summary>
/// The state for the app feature.
/// </summary>
[FeatureState]
public record AppState
{
    /// <summary>
    /// The selected page to view.
    /// </summary>
    public SelectedPage SelectedPage { get; init; } = SelectedPage.AccountSelect;
}