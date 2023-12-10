using BetterMoneyManager.Models;
using BetterMoneyManager.Store.AccountSlice;
using Fluxor;

namespace BetterMoneyManager.Store.AppSlice;

public class AppEffects
{
    [EffectMethod]
    public async Task OnEditAccount(EditAccountRequestedAction action, IDispatcher dispatcher)
    {
        dispatcher.Dispatch(new SelectPageAction(SelectedPage.EditAccount));
    }
}