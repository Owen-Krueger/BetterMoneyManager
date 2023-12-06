namespace BetterMoneyManager.Models;

public class Account
{
    public int Id { get; set; }

    public string Name { get; set; }

    public string BankName { get; set; }

    public int AccountType { get; set; }

    public decimal Balance { get; set; }

    public decimal AvailableBalance { get; set; }

    public bool Favorite { get; set; }

    public DateTime DateCreated { get; set; }
}