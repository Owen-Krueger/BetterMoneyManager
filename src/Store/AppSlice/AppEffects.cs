using BetterMoneyManager.Models;
using BetterMoneyManager.Store.AccountSlice;
using Fluxor;

namespace BetterMoneyManager.Store.AppSlice;

/// <summary>
/// Effects to run when app actions are dispatched.
/// </summary>
public class AppEffects
{
    /// <summary>
    /// On account edit requested, display <see cref="SelectedPage.EditAccount"/> page.
    /// </summary>
    /// <param name="dispatcher">Dispatch event when account edit requested.</param>
    [EffectMethod(typeof(EditAccountRequestedAction))]
    public async Task OnEditAccountRequested(IDispatcher dispatcher)
    {
        dispatcher.Dispatch(new SelectPageAction(SelectedPage.EditAccount));
    }
}