using Fluxor;

namespace BetterMoneyManager.Store.AppSlice;

/// <summary>
/// Reducer methods to handle app actions.
/// </summary>
public class AppReducers
{
    /// <summary>
    /// Requests to set the selected page.
    /// </summary>
    /// <param name="state">App state.</param>
    /// <param name="action">The page to display.</param>
    /// <returns>Updated app state.</returns>
    [ReducerMethod]
    public static AppState OnSelectPage(AppState state, SelectPageAction action)
    {
        return state with { SelectedPage = action.SelectedPage };
    }
}