using BetterMoneyManager.Models;

namespace BetterMoneyManager.Store.AppSlice;

/// <summary>
/// Selects a page to display.
/// </summary>
/// <param name="SelectedPage">The page to display.</param>
public record SelectPageAction(SelectedPage SelectedPage);