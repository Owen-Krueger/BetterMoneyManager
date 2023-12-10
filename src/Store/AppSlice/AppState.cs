using BetterMoneyManager.Models;
using Fluxor;

namespace BetterMoneyManager.Store.AppSlice;

[FeatureState]
public record AppState
{
    public SelectedPage SelectedPage { get; set; } = SelectedPage.AccountSelect;
}