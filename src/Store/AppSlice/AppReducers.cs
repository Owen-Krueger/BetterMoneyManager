using Fluxor;

namespace BetterMoneyManager.Store.AppSlice;

public class AppReducers
{
    [ReducerMethod]
    public static AppState OnSelectPage(AppState state, SelectPageAction action)
    {
        return state with { SelectedPage = action.SelectedPage };
    }
}